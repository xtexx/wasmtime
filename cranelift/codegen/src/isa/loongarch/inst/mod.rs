//! This module defines LA-specific machine instruction types.

use crate::binemit::{Addend, CodeOffset, Reloc};
pub use crate::ir::condcodes::IntCC;
use crate::ir::types::{F16, F32, F64, F128, I8, I16, I32, I64, I128};

pub use crate::ir::{ExternalName, MemFlagsData, Type};
use crate::isa::{CallConv, FunctionAlignment};
use crate::machinst::*;
use crate::{CodegenError, CodegenResult, settings};

pub use crate::ir::condcodes::FloatCC;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;
use core::ops::{BitAnd, Not, Shl};
use regalloc2::RegClass;
use smallvec::{SmallVec, smallvec};

pub mod regs;
pub use self::regs::*;
pub mod args;
pub use self::args::*;
pub mod emit;
pub use self::emit::*;
pub mod imm;
pub use self::imm::*;
pub mod encode;
pub use self::encode::*;

use crate::isa::loongarch::abi::LoongArchMachineDeps;
pub use crate::isa::loongarch::lower::isle::generated_code::MInst;
use crate::isa::loongarch::lower::isle::generated_code::*;

use core::fmt::{Display, Formatter};

pub(crate) type VecU8 = Vec<u8>;

pub(crate) fn is_uimm_fit<N>(value: N, width: u8) -> bool
where
    N: Shl<u8, Output = N>,
    N: From<u8>,
    N: Not<Output = N>,
    N: PartialEq + Eq,
    N: BitAnd<Output = N>,
{
    (value & (!N::from(0) << width)) == N::from(0)
}

pub(crate) fn is_simm_fit<N>(value: N, width: u8) -> bool
where
    N: Shl<u8, Output = N>,
    N: From<u8>,
    N: Not<Output = N>,
    N: PartialEq + Eq,
    N: BitAnd<Output = N>,
{
    let t = value & (!N::from(0) << (width - 1));
    t == N::from(0) || t == !N::from(0)
}

/// Additional information for `return_call[_ind]` instructions, left out of
/// line to lower the size of the `MInst` enum.
#[derive(Clone, Debug)]
pub struct ReturnCallInfo<T> {
    pub dest: T,
    pub uses: CallArgList,
    pub new_stack_arg_size: u32,
}

/// A conditional branch target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CondBrTarget {
    /// An unresolved reference to a Label, as passed into
    /// `lower_branch_group()`.
    Label(MachLabel),
    /// No jump; fall through to the next instruction.
    Fallthrough,
}

impl CondBrTarget {
    /// Return the target's label, if it is a label-based target.
    pub(crate) fn as_label(self) -> Option<MachLabel> {
        match self {
            CondBrTarget::Label(l) => Some(l),
            _ => None,
        }
    }

    pub(crate) fn is_fallthrouh(&self) -> bool {
        self == &CondBrTarget::Fallthrough
    }
}

impl Display for CondBrTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            CondBrTarget::Label(l) => write!(f, "{}", l.to_string()),
            CondBrTarget::Fallthrough => write!(f, "0"),
        }
    }
}

impl MInst {
    pub fn load_imm(rd: Writable<Reg>, value: u64) -> MInst {
        MInst::LoadImmGpr {
            dst: rd,
            imm: value,
        }
    }

    /// Generic constructor for a load (zero-extending where appropriate).
    pub fn gen_load(into_reg: Writable<Reg>, mem: AMode, ty: Type, flags: MemFlagsData) -> MInst {
        assert!(!ty.is_vector()); // TODO vector load
        MInst::Load {
            rd: into_reg,
            op: LoadOP::from_type(ty),
            from: mem,
            flags,
        }
    }

    /// Generic constructor for a store.
    pub fn gen_store(mem: AMode, from_reg: Reg, ty: Type, flags: MemFlagsData) -> MInst {
        assert!(!ty.is_vector()); // TODO vector store
        MInst::Store {
            src: from_reg,
            op: StoreOP::from_type(ty),
            to: mem,
            flags,
        }
    }
}

impl From<LInst> for MInst {
    fn from(inst: LInst) -> Self {
        Self::LInst { inst }
    }
}

impl MachInst for MInst {
    type LabelUse = LabelUse;
    type ABIMachineSpec = LoongArchMachineDeps;

    const TRAP_OPCODE: &'static [u8] = &[0; 4];

    fn gen_dummy_use(reg: Reg) -> Self {
        MInst::DummyUse { reg }
    }

    fn canonical_type_for_rc(rc: RegClass) -> Type {
        match rc {
            regalloc2::RegClass::Int => I64,
            regalloc2::RegClass::Float => F64,
            regalloc2::RegClass::Vector => LA_FCC_TYPE,
        }
    }

    fn is_safepoint(&self) -> bool {
        match self {
            MInst::Call { .. } | MInst::CallInd { .. } => true,
            _ => false,
        }
    }

    fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            MInst::Nop0 | MInst::Nop4 => {}
            MInst::DummyUse { reg } => collector.reg_use(reg),
            MInst::LoadImmGpr { dst, .. } => collector.reg_def(dst),
            MInst::LInst { inst } => get_linst_operands(inst, collector),
            MInst::Move { rd, rj, .. } => {
                collector.reg_use(rj);
                collector.reg_def(rd);
            }
            MInst::EmitIsland { .. } | MInst::Trap => {}
            MInst::Call { info, .. } => {
                let CallInfo { uses, defs, .. } = &mut **info;
                for CallArgPair { vreg, preg } in uses {
                    collector.reg_fixed_use(vreg, *preg);
                }
                for CallRetPair { vreg, location } in defs {
                    match location {
                        RetLocation::Reg(preg, ..) => collector.reg_fixed_def(vreg, *preg),
                        RetLocation::Stack(..) => collector.any_def(vreg),
                    }
                }
                collector.reg_clobbers(info.clobbers);
                if let Some(try_call_info) = &mut info.try_call_info {
                    try_call_info.collect_operands(collector);
                }
            }
            MInst::CallInd { info } => {
                let CallInfo {
                    dest, uses, defs, ..
                } = &mut **info;
                collector.reg_use(dest);
                for CallArgPair { vreg, preg } in uses {
                    collector.reg_fixed_use(vreg, *preg);
                }
                for CallRetPair { vreg, location } in defs {
                    match location {
                        RetLocation::Reg(preg, ..) => collector.reg_fixed_def(vreg, *preg),
                        RetLocation::Stack(..) => collector.any_def(vreg),
                    }
                }
                collector.reg_clobbers(info.clobbers);
                if let Some(try_call_info) = &mut info.try_call_info {
                    try_call_info.collect_operands(collector);
                }
            }
            MInst::ReturnCall { info } => {
                for CallArgPair { vreg, preg } in &mut info.uses {
                    collector.reg_fixed_use(vreg, *preg);
                }
            }
            MInst::ReturnCallInd { info } => {
                collector.reg_fixed_use(&mut info.dest, x_reg(1));
                for CallArgPair { vreg, preg } in &mut info.uses {
                    collector.reg_fixed_use(vreg, *preg);
                }
            }
            MInst::Args { args } => {
                for ArgPair { vreg, preg } in args {
                    collector.reg_fixed_def(vreg, *preg);
                }
            }
            MInst::Rets { rets } => {
                for RetPair { vreg, preg } in rets {
                    collector.reg_fixed_use(vreg, *preg);
                }
            }
            MInst::Branch { .. } | MInst::Ret => {}
            MInst::CondBr {
                kind: IntegerCompare { rs1, rs2, .. },
                ..
            } => {
                collector.reg_use(rs1);
                collector.reg_use(rs2);
            }
            MInst::Select {
                dst,
                condition,
                x,
                y,
            } => {
                collector.reg_late_use(&mut condition.rs1);
                collector.reg_late_use(&mut condition.rs2);

                for reg in x.regs_mut() {
                    collector.reg_use(reg);
                }
                for reg in y.regs_mut() {
                    collector.reg_use(reg);
                }

                match dst.regs_mut() {
                    [reg] => collector.reg_def(reg),
                    regs => {
                        for d in regs {
                            collector.reg_early_def(d);
                        }
                    }
                }
            }
            MInst::BrTable {
                index, tmp1, tmp2, ..
            } => {
                collector.reg_use(index);
                collector.reg_early_def(tmp1);
                collector.reg_early_def(tmp2);
            }
            MInst::StackProbeLoop { .. } => {}
            MInst::Load { rd, from, .. } => {
                from.get_operands(collector);
                collector.reg_def(rd);
            }
            MInst::Store { to, src, .. } => {
                to.get_operands(collector);
                collector.reg_use(src);
            }
            MInst::LoadExtNameGot { rd, .. } | MInst::LoadExtNameNear { rd, .. } => {
                collector.reg_def(rd)
            }
            MInst::LoadExtNameFar { rd, tmp_reg, .. } => {
                collector.reg_def(tmp_reg);
                collector.reg_def(rd);
            }
        }
    }

    fn is_move(&self) -> Option<(Writable<Reg>, Reg)> {
        match self {
            MInst::Move { rd, rj, .. } => Some((*rd, *rj)),
            _ => None,
        }
    }

    fn is_included_in_clobbers(&self) -> bool {
        !matches!(self, MInst::Args { .. })
    }

    fn is_trap(&self) -> bool {
        matches!(self, MInst::Trap)
    }

    fn is_args(&self) -> bool {
        matches!(self, MInst::Args { .. })
    }

    fn call_type(&self) -> CallType {
        match self {
            MInst::Call { .. } | MInst::CallInd { .. } => CallType::Regular,

            MInst::ReturnCall { .. } | MInst::ReturnCallInd { .. } => CallType::TailCall,

            _ => CallType::None,
        }
    }

    fn is_term(&self) -> MachTerminator {
        match self {
            MInst::Branch { .. } => MachTerminator::Branch,
            MInst::CondBr { .. } => MachTerminator::Branch,
            MInst::Rets { .. } | MInst::Ret { .. } => MachTerminator::Ret,
            // &MInst::BrTable { .. } => MachTerminator::Branch,
            MInst::ReturnCall { .. } | MInst::ReturnCallInd { .. } => MachTerminator::RetCall,
            MInst::Call { info } if info.try_call_info.is_some() => MachTerminator::Branch,
            MInst::CallInd { info } if info.try_call_info.is_some() => MachTerminator::Branch,
            _ => MachTerminator::None,
        }
    }

    fn is_mem_access(&self) -> bool {
        panic!("TODO FILL ME OUT")
    }

    fn gen_move(to_reg: Writable<Reg>, from_reg: Reg, ty: Type) -> MInst {
        MInst::Move {
            rd: to_reg,
            rj: from_reg,
            ty,
        }
    }

    fn gen_nop(preferred_size: usize) -> MInst {
        if preferred_size == 0 {
            return MInst::Nop0;
        }
        assert!(preferred_size >= 4);
        MInst::Nop4
    }

    fn gen_nop_units() -> Vec<Vec<u8>> {
        vec![vec![0x03, 0x40, 0x00, 0x00]]
    }

    fn rc_for_type(ty: Type) -> CodegenResult<(&'static [RegClass], &'static [Type])> {
        match ty {
            I8 => Ok((&[RegClass::Int], &[I8])),
            I16 => Ok((&[RegClass::Int], &[I16])),
            I32 => Ok((&[RegClass::Int], &[I32])),
            I64 => Ok((&[RegClass::Int], &[I64])),
            I128 => Ok((&[RegClass::Int, RegClass::Int], &[I128])),
            F16 => Ok((&[RegClass::Float], &[F16])),
            F32 => Ok((&[RegClass::Float], &[F32])),
            F64 => Ok((&[RegClass::Float], &[F64])),
            F128 => Ok((&[RegClass::Float, RegClass::Float], &[F128])), // LSX?
            LA_FCC_TYPE => Ok((&[RegClass::Vector], &[LA_FCC_TYPE])),

            _ => Err(CodegenError::Unsupported(format!(
                "Unexpected SSA-value type: {ty}"
            ))),
        }
    }

    fn gen_jump(target: MachLabel) -> MInst {
        MInst::Branch { label: target }
    }

    fn worst_case_size() -> CodeOffset {
        20
    }

    fn worst_case_island_growth() -> CodeOffset {
        // Conservative upper bound on per-instruction additions to the
        // buffer's pending-island state: several embedded constants, a
        // deferred trap, and a handful of fixups whose worst-case veneers
        // are 8 bytes apiece. Sized comfortably to cover real cases.
        128
    }

    fn ref_type_regclass(_settings: &settings::Flags) -> RegClass {
        RegClass::Int
    }

    fn function_alignment() -> FunctionAlignment {
        FunctionAlignment {
            minimum: 2,
            preferred: 4,
        }
    }
}

fn pretty_print_try_call(info: &TryCallInfo) -> String {
    format!(
        "; j {:?}; catch [{}]",
        info.continuation,
        info.pretty_print_dests()
    )
}

fn format_reg(reg: Reg) -> String {
    match reg.class() {
        RegClass::Int => format!("$r{}", reg_to_num(reg)),
        RegClass::Float => format!("$f{}", reg_to_num(reg)),
        RegClass::Vector => format!("$fcc{}", reg_to_num(reg)),
    }
}

/// Type of a floating-point comparison result
pub(crate) const LA_FCC_TYPE: Type = Type::from_repr(0x01);

impl MInst {
    fn print_with_state(&self, _state: &mut EmitState) -> String {
        let format_regs = |regs: &[Reg]| -> String {
            let mut x = if regs.len() > 1 {
                String::from("[")
            } else {
                String::default()
            };
            regs.iter().for_each(|i| {
                x.push_str(format_reg(*i).as_str());
                if *i != *regs.last().unwrap() {
                    x.push_str(",");
                }
            });
            if regs.len() > 1 {
                x.push_str("]");
            }
            x
        };
        let format_labels = |labels: &[MachLabel]| -> String {
            if labels.len() == 0 {
                return String::from("[_]");
            }
            let mut x = String::from("[");
            labels.iter().for_each(|l| {
                x.push_str(
                    format!(
                        "{:?}{}",
                        l,
                        if l != labels.last().unwrap() { "," } else { "" },
                    )
                    .as_str(),
                );
            });
            x.push_str("]");
            x
        };

        match self {
            MInst::Nop0 => {
                format!("# zero length nop")
            }
            MInst::Nop4 => {
                format!("nop")
            }
            MInst::LInst { inst } => {
                let mut buf = String::new();
                format_linst(inst, &mut buf).unwrap();
                buf
            }
            MInst::Call { info } => {
                let try_call = info
                    .try_call_info
                    .as_ref()
                    .map(|tci| pretty_print_try_call(tci))
                    .unwrap_or_default();
                format!("call {}{try_call}", info.dest.display(None))
            }
            MInst::CallInd { info } => {
                let rd = format_reg(info.dest);
                let try_call = info
                    .try_call_info
                    .as_ref()
                    .map(|tci| pretty_print_try_call(tci))
                    .unwrap_or_default();
                format!("callind {rd}{try_call}")
            }
            MInst::ReturnCall { info } => {
                let mut s = format!(
                    "return_call {:?} new_stack_arg_size:{}",
                    info.dest, info.new_stack_arg_size
                );
                for ret in &info.uses {
                    let preg = format_reg(ret.preg);
                    let vreg = format_reg(ret.vreg);
                    write!(&mut s, " {vreg}={preg}").unwrap();
                }
                s
            }
            MInst::ReturnCallInd { info } => {
                let callee = format_reg(info.dest);
                let mut s = format!(
                    "return_call_ind {callee} new_stack_arg_size:{}",
                    info.new_stack_arg_size
                );
                for ret in &info.uses {
                    let preg = format_reg(ret.preg);
                    let vreg = format_reg(ret.vreg);
                    write!(&mut s, " {vreg}={preg}").unwrap();
                }
                s
            }
            MInst::Args { args } => {
                let mut s = "args".to_string();
                for arg in args {
                    let preg = format_reg(arg.preg);
                    let def = format_reg(arg.vreg.to_reg());
                    write!(&mut s, " {def}={preg}").unwrap();
                }
                s
            }
            MInst::Rets { rets } => {
                let mut s = "rets".to_string();
                for ret in rets {
                    let preg = format_reg(ret.preg);
                    let vreg = format_reg(ret.vreg);
                    write!(&mut s, " {vreg}={preg}").unwrap();
                }
                s
            }
            MInst::Ret {} => "ret".to_string(),
            MInst::DummyUse { reg } => {
                let reg = format_reg(*reg);
                format!("dummy_use {reg}")
            }
            MInst::Move { rd, rj, ty } => {
                format!("move $r{}, $r{}", reg_to_num(rd.to_reg()), reg_to_num(*rj))
            }
            MInst::Branch { label } => {
                format!("b {}", label.to_string())
            }
            MInst::CondBr {
                taken,
                not_taken,
                kind,
            } => {
                let rs1 = format_reg(kind.rs1);
                let rs2 = format_reg(kind.rs2);
                if not_taken.is_fallthrouh() && taken.as_label().is_none() {
                    format!("cond_br {}, {}, {}, 0", kind.kind, rs1, rs2)
                } else {
                    let x = format!(
                        "cond_br {}, {}, {}, taken({}), not_taken({})",
                        kind.kind,
                        rs1,
                        rs2,
                        taken,
                        not_taken
                    );
                    x
                }
            }
            MInst::Trap => String::from("trap"),
            MInst::EmitIsland { needed_space } => {
                format!("emit_island {needed_space}")
            }
            MInst::LoadImmGpr { dst, imm } => {
                format!("li.d $r{}, {}", reg_to_num(dst.to_reg()), imm)
            }
            MInst::Select {
                dst,
                condition,
                x,
                y,
            } => {
                let c_rs1 = format_reg(condition.rs1);
                let c_rs2 = format_reg(condition.rs2);
                let x = format_regs(x.regs());
                let y = format_regs(y.regs());
                let dst = dst.map(|r| r.to_reg());
                let dst = format_regs(dst.regs());
                format!(
                    "select {},{},{}##condition=({} {} {})",
                    dst,
                    x,
                    y,
                    c_rs1,
                    condition.kind.to_static_str(),
                    c_rs2
                )
            }
            MInst::BrTable {
                index,
                tmp1,
                tmp2,
                targets,
            } => {
                format!(
                    "{} {},{}##tmp1={},tmp2={}",
                    "br_table",
                    format_reg(*index),
                    format_labels(&targets[..]),
                    format_reg(tmp1.to_reg()),
                    format_reg(tmp2.to_reg()),
                )
            }
            MInst::StackProbeLoop {
                guard_size,
                probe_count,
                tmp,
            } => {
                let tmp = format_reg(tmp.to_reg());
                format!(
                    "inline_stack_probe guard_size={guard_size} probe_count={probe_count} tmp={tmp}"
                )
            }
            MInst::Load { rd, op, from, .. } => {
                let base = from.to_string();
                let rd = format_reg(rd.to_reg());
                format!("{} {},{}", op.op_name(), rd, base,)
            }
            MInst::Store { to, src, op, .. } => {
                let base = to.to_string();
                let src = format_reg(*src);
                format!("{} {},{}", op.op_name(), src, base)
            }
            MInst::LoadExtNameGot { rd, name } => {
                let rd = format_reg(rd.to_reg());
                format!("load_ext_name_got {rd}, {}", name.display(None))
            }
            MInst::LoadExtNameNear { rd, name, offset } => {
                let rd = format_reg(rd.to_reg());
                format!("load_ext_name_near {rd}, {}{offset:+}", name.display(None))
            }
            MInst::LoadExtNameFar {
                rd,
                name,
                offset,
                tmp_reg,
            } => {
                let rd = format_reg(rd.to_reg());
                let tmp_reg = format_reg(tmp_reg.to_reg());
                format!(
                    "load_ext_name_far {rd}, {}{offset:+} tmp={tmp_reg}",
                    name.display(None)
                )
            }
        }
    }
}

/// Different forms of label references for different instruction formats.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelUse {
    B16,
    B21,
    B26,
}

impl MachInstLabelUse for LabelUse {
    /// Alignment for veneer code. Every instruction must be
    /// 4-byte-aligned.
    const ALIGN: CodeOffset = 4;

    /// Maximum PC-relative range (positive), inclusive.
    fn max_pos_range(self) -> CodeOffset {
        match self {
            LabelUse::B16 => ((1 << 15) - 1) * 4,
            LabelUse::B21 => ((1 << 20) - 1) * 4,
            LabelUse::B26 => ((1 << 25) - 1) * 4,
        }
    }

    /// Maximum PC-relative range (negative).
    fn max_neg_range(self) -> CodeOffset {
        match self {
            _ => self.max_pos_range() + 2,
        }
    }

    /// Size of window into code needed to do the patch.
    fn patch_size(self) -> CodeOffset {
        match self {
            LabelUse::B16 | LabelUse::B21 | LabelUse::B26 => 4,
        }
    }

    /// Perform the patch.
    fn patch(self, buffer: &mut [u8], use_offset: CodeOffset, label_offset: CodeOffset) {
        assert!(use_offset % 2 == 0);
        assert!(label_offset % 2 == 0);
        let offset = (label_offset as i64) - (use_offset as i64);

        // re-check range
        assert!(
            offset >= -(self.max_neg_range() as i64) && offset <= (self.max_pos_range() as i64),
            "{self:?} offset '{offset}' use_offset:'{use_offset}' label_offset:'{label_offset}'  must not exceed max range.",
        );
        self.patch_raw_offset(buffer, offset);
    }

    /// Is a veneer supported for this label reference type?
    fn supports_veneer(self) -> bool {
        false // TODO trampoline support
    }

    /// How large is the veneer, if supported?
    fn veneer_size(self) -> CodeOffset {
        0
    }

    fn worst_case_veneer_size() -> CodeOffset {
        0
    }

    /// Generate a veneer into the buffer, given that this veneer is at `veneer_offset`, and return
    /// an offset and label-use for the veneer's use of the original label.
    fn generate_veneer(
        self,
        _buffer: &mut [u8],
        _veneer_offset: CodeOffset,
    ) -> (CodeOffset, LabelUse) {
        unreachable!()
    }

    fn from_reloc(reloc: Reloc, addend: Addend) -> Option<LabelUse> {
        match (reloc, addend) {
            (Reloc::LoongArchB16, _) => Some(Self::B16),
            (Reloc::LoongArchB21, _) => Some(Self::B21),
            (Reloc::LoongArchB26, _) => Some(Self::B26),
            _ => None,
        }
    }
}

impl LabelUse {
    fn patch_raw_offset(self, buffer: &mut [u8], offset: i64) {
        let insn = u32::from_le_bytes(buffer[..4].try_into().unwrap());

        match self {
            LabelUse::B16 => {
                let offset = offset as u32;
                let insn = (insn & !0x3FFFC00) | (offset << 10);
                buffer[0..4].clone_from_slice(&u32::to_le_bytes(insn));
            }
            LabelUse::B21 => {
                let offset = offset as u32;
                let insn = (insn & !0x3FFFC1F) | ((offset & 0xFFFF) << 10) | (offset >> 16);
                buffer[0..4].clone_from_slice(&u32::to_le_bytes(insn));
            }
            LabelUse::B26 => {
                let offset = offset as u32;
                let insn = (insn & !0x3FFFFFF) | ((offset & 0xFFFF) << 10) | (offset >> 16);
                buffer[0..4].clone_from_slice(&u32::to_le_bytes(insn));
            }
        }
    }
}
