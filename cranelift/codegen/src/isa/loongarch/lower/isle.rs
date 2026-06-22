//! ISLE integration glue code for LoongArch lowering.

// Pull in the ISLE generated code.
pub mod generated_code;
use generated_code::MInst;

// Types that the generated ISLE code uses via `use super::*`.
use crate::isa::loongarch::LoongArchBackend;
use crate::machinst::Reg;
use crate::machinst::{CallInfo, MachInst, isle::*};
use crate::machinst::{VCodeConstant, VCodeConstantData};
use crate::{
    ir::{
        BlockCall, ExternalName, Inst, InstructionData, MemFlagsData, Opcode, TrapCode, Value,
        ValueList, immediates::*, types::*,
    },
    isa::loongarch::inst::*,
    machinst::{ArgPair, CallArgList, CallRetList, InstOutput},
};
use alloc::boxed::Box;
use alloc::vec::Vec;
use regalloc2::PReg;
use wasmtime_core::math::{f32_cvt_to_int_bounds, f64_cvt_to_int_bounds};

type BoxCallInfo = Box<CallInfo<ExternalName>>;
type BoxCallIndInfo = Box<CallInfo<Reg>>;
type BoxReturnCallInfo = Box<ReturnCallInfo<ExternalName>>;
type BoxReturnCallIndInfo = Box<ReturnCallInfo<Reg>>;
type BoxExternalName = Box<ExternalName>;
type VecMachLabel = Vec<MachLabel>;
type VecArgPair = Vec<ArgPair>;

pub(crate) struct LAIsleContext<'a, 'b, I, B>
where
    I: VCodeInst,
    B: LowerBackend,
{
    pub lower_ctx: &'a mut Lower<'b, I>,
    pub backend: &'a B,
}

impl<'a, 'b> LAIsleContext<'a, 'b, MInst, LoongArchBackend> {
    fn new(lower_ctx: &'a mut Lower<'b, MInst>, backend: &'a LoongArchBackend) -> Self {
        Self { lower_ctx, backend }
    }

    pub(crate) fn dfg(&self) -> &crate::ir::DataFlowGraph {
        &self.lower_ctx.f.dfg
    }
}

impl generated_code::Context for LAIsleContext<'_, '_, MInst, LoongArchBackend> {
    isle_lower_prelude_methods!();

    #[inline]
    fn emit(&mut self, arg0: &MInst) -> Unit {
        self.lower_ctx.emit(arg0.clone());
    }

    #[inline]
    fn is_pic(&mut self) -> bool {
        self.backend.flags.is_pic()
    }

    #[inline]
    fn label_to_br_target(&mut self, label: MachLabel) -> CondBrTarget {
        CondBrTarget::Label(label)
    }

    #[inline]
    fn has_f(&mut self) -> bool {
        self.backend.isa_flags.has_f()
    }

    #[inline]
    fn has_d(&mut self) -> bool {
        self.backend.isa_flags.has_d()
    }

    fn gen_call_info(
        &mut self,
        sig: Sig,
        dest: ExternalName,
        uses: CallArgList,
        defs: CallRetList,
        try_call_info: Option<TryCallInfo>,
        patchable: bool,
    ) -> BoxCallInfo {
        let stack_ret_space = self.lower_ctx.sigs()[sig].sized_stack_ret_space();
        let stack_arg_space = self.lower_ctx.sigs()[sig].sized_stack_arg_space();
        self.lower_ctx
            .abi_mut()
            .accumulate_outgoing_args_size(stack_ret_space + stack_arg_space);

        Box::new(
            self.lower_ctx
                .gen_call_info(sig, dest, uses, defs, try_call_info, patchable),
        )
    }

    fn gen_call_ind_info(
        &mut self,
        sig: Sig,
        dest: Reg,
        uses: CallArgList,
        defs: CallRetList,
        try_call_info: Option<TryCallInfo>,
    ) -> BoxCallIndInfo {
        let stack_ret_space = self.lower_ctx.sigs()[sig].sized_stack_ret_space();
        let stack_arg_space = self.lower_ctx.sigs()[sig].sized_stack_arg_space();
        self.lower_ctx
            .abi_mut()
            .accumulate_outgoing_args_size(stack_ret_space + stack_arg_space);

        Box::new(
            self.lower_ctx
                .gen_call_info(sig, dest, uses, defs, try_call_info, false),
        )
    }

    fn gen_return_call_info(
        &mut self,
        sig: Sig,
        dest: ExternalName,
        uses: CallArgList,
    ) -> BoxReturnCallInfo {
        let new_stack_arg_size = self.lower_ctx.sigs()[sig].sized_stack_arg_space();
        self.lower_ctx
            .abi_mut()
            .accumulate_tail_args_size(new_stack_arg_size);

        Box::new(ReturnCallInfo {
            dest,
            uses,
            new_stack_arg_size,
        })
    }

    fn gen_return_call_ind_info(
        &mut self,
        sig: Sig,
        dest: Reg,
        uses: CallArgList,
    ) -> BoxReturnCallIndInfo {
        let new_stack_arg_size = self.lower_ctx.sigs()[sig].sized_stack_arg_space();
        self.lower_ctx
            .abi_mut()
            .accumulate_tail_args_size(new_stack_arg_size);

        Box::new(ReturnCallInfo {
            dest,
            uses,
            new_stack_arg_size,
        })
    }

    #[inline]
    fn int_compare(&mut self, kind: &IntCC, rs1: Reg, rs2: Reg) -> IntegerCompare {
        IntegerCompare {
            kind: *kind,
            rs1,
            rs2,
        }
    }

    #[inline]
    fn int_compare_inverse(&mut self, c: IntegerCompare) -> IntegerCompare {
        c.inverse()
    }

    #[inline]
    fn int_compare_decompose(&mut self, cmp: IntegerCompare) -> (IntCC, Reg, Reg) {
        (cmp.kind, cmp.rs1, cmp.rs2)
    }

    #[inline]
    fn writable_zero_reg(&mut self) -> WritableReg {
        writable_zero_reg()
    }

    #[inline]
    fn zero_reg(&mut self) -> Reg {
        zero_reg()
    }

    #[inline]
    fn is_zero_reg(&mut self, reg: Reg) -> Option<()> {
        if reg == zero_reg() { Some(()) } else { None }
    }

    #[inline]
    fn fp_reg(&mut self) -> PReg {
        px_reg(22)
    }

    #[inline]
    fn sp_reg(&mut self) -> PReg {
        px_reg(3)
    }

    #[inline]
    fn fits_in_gpr(&mut self, ty: Type) -> Option<Type> {
        if ty.bytes() <= 64 {
            // TODO: LA32
            Some(ty)
        } else {
            None
        }
    }

    fn ty_supported(&mut self, ty: Type) -> Option<Type> {
        let supported = match ty {
            ty if ty.is_int() && ty != I128 => true,
            // F32 depends on the F extension
            F32 => self.backend.isa_flags.has_f(),
            // F64 depends on the D extension
            F64 => self.backend.isa_flags.has_d(),
            // F16 is not implemented yet
            F16 => false,
            // F128 is currently stored in a pair of integer registers
            F128 => true,

            // Otherwise do not match
            _ => false,
        };

        if supported { Some(ty) } else { None }
    }

    fn ty_supported_float(&mut self, ty: Type) -> Option<Type> {
        self.ty_supported(ty)
            .filter(|&ty| ty.is_float() && ty != F128)
    }

    fn ty_supported_vec(&mut self, ty: Type) -> Option<Type> {
        self.ty_supported(ty).filter(|ty| ty.is_vector())
    }

    fn ty_reg_pair(&mut self, ty: Type) -> Option<Type> {
        match ty {
            I128 | F128 => Some(ty),
            _ => None,
        }
    }

    #[inline]
    fn uimm(&mut self, n: u32) -> Uimm {
        Uimm::from_u32(n)
    }

    #[inline]
    fn simm(&mut self, n: i32) -> Simm {
        Simm::from_i32(n)
    }

    #[inline]
    fn uimm12_from_u32(&mut self, n: u32) -> Option<Uimm> {
        self.uimm_from_u32(n, 12)
    }

    #[inline]
    fn simm12_from_i32(&mut self, n: i32) -> Option<Simm> {
        self.simm_from_i32(n, 12)
    }

    #[inline]
    fn uimm6_from_u32(&mut self, n: u32) -> Option<Uimm> {
        self.uimm_from_u32(n, 6)
    }

    #[inline]
    fn simm6_from_i32(&mut self, n: i32) -> Option<Simm> {
        self.simm_from_i32(n, 6)
    }
}

impl LAIsleContext<'_, '_, MInst, LoongArchBackend> {
    pub(crate) fn uimm_from_u32(&mut self, n: u32, bits: u8) -> Option<Uimm> {
        if is_uimm_fit(n, bits) {
            Some(Uimm::from_u32(n))
        } else {
            None
        }
    }
    pub(crate) fn simm_from_i32(&mut self, n: i32, bits: u8) -> Option<Simm> {
        if is_simm_fit(n, bits) {
            Some(Simm::from_i32(n))
        } else {
            None
        }
    }
}

/// The main entry point for lowering with ISLE.
pub(crate) fn lower(
    lower_ctx: &mut Lower<MInst>,
    backend: &LoongArchBackend,
    inst: Inst,
) -> Option<InstOutput> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = LAIsleContext::new(lower_ctx, backend);
    generated_code::constructor_lower(&mut isle_ctx, inst)
}

/// The main entry point for branch lowering with ISLE.
pub(crate) fn lower_branch(
    lower_ctx: &mut Lower<MInst>,
    backend: &LoongArchBackend,
    branch: Inst,
    targets: &[MachLabel],
) -> Option<()> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = LAIsleContext::new(lower_ctx, backend);
    generated_code::constructor_lower_branch(&mut isle_ctx, branch, targets)
}
