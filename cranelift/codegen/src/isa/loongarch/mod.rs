//! LoongArch Instruction Set Architecture.
//! 
//! NOTE: This backend uses RegClass::Float to represent vector registers
//! and RegClass::Vector to represent FCC registers.

use crate::dominator_tree::DominatorTree;
use crate::ir::{Function, Type};
use crate::isa::loongarch::inst::EmitInfo;
use crate::isa::loongarch::settings as la_settings;
use crate::isa::{
    Builder as IsaBuilder, FunctionAlignment, IsaFlagsHashKey, OwnedTargetIsa, TargetIsa,
};
#[cfg(feature = "unwind")]
use crate::machinst::CompiledCode;
use crate::machinst::{
    CompiledCodeStencil, MachInst, MachTextSectionBuilder, Reg, SigSet, TextSectionBuilder, VCode,
    compile,
};
use crate::result::CodegenResult;
use crate::settings::{self as shared_settings, Flags};
use crate::{CodegenError, ir};
use alloc::string::String;
use alloc::{boxed::Box, vec::Vec};
use core::fmt;
use cranelift_control::ControlPlane;
use target_lexicon::{Architecture, Triple};
mod abi;
pub(crate) mod inst;
mod lower;
mod settings;
#[cfg(feature = "unwind")]
use crate::isa::unwind::systemv;

/// An LoongArch backend.
pub struct LoongArchBackend {
    triple: Triple,
    flags: shared_settings::Flags,
    isa_flags: la_settings::Flags,
}

impl LoongArchBackend {
    /// Create a new LoongArch backend with the given (shared) flags.
    pub fn new_with_flags(
        triple: Triple,
        flags: shared_settings::Flags,
        isa_flags: la_settings::Flags,
    ) -> LoongArchBackend {
        LoongArchBackend {
            triple,
            flags,
            isa_flags,
        }
    }

    /// This performs lowering to VCode, register-allocates the code, computes block layout and
    /// finalizes branches. The result is ready for binary emission.
    fn compile_vcode(
        &self,
        func: &Function,
        domtree: &DominatorTree,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<(VCode<inst::MInst>, regalloc2::Output)> {
        let emit_info = EmitInfo::new(self.flags.clone(), self.isa_flags.clone());
        let sigs = SigSet::new::<abi::LoongArchMachineDeps>(func, &self.flags)?;
        let abi = abi::LoongArchCallee::new(func, self, &self.isa_flags, &sigs)?;
        compile::compile::<LoongArchBackend>(func, domtree, self, abi, emit_info, sigs, ctrl_plane)
    }
}

impl TargetIsa for LoongArchBackend {
    fn compile_function(
        &self,
        func: &Function,
        domtree: &DominatorTree,
        want_disasm: bool,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<CompiledCodeStencil> {
        let (vcode, regalloc_result) = self.compile_vcode(func, domtree, ctrl_plane)?;

        let want_disasm = want_disasm || log::log_enabled!(log::Level::Debug);
        let emit_result = vcode.emit(&regalloc_result, want_disasm, &self.flags, ctrl_plane);
        let value_labels_ranges = emit_result.value_labels_ranges;
        let buffer = emit_result.buffer;

        if let Some(disasm) = emit_result.disasm.as_ref() {
            log::debug!("disassembly:\n{disasm}");
        }

        Ok(CompiledCodeStencil {
            buffer,
            vcode: emit_result.disasm,
            value_labels_ranges,
            bb_starts: emit_result.bb_offsets,
            bb_edges: emit_result.bb_edges,
        })
    }

    fn name(&self) -> &'static str {
        "loongarch"
    }

    fn dynamic_vector_bytes(&self, _dynamic_ty: ir::Type) -> u32 {
        16
    }

    fn triple(&self) -> &Triple {
        &self.triple
    }

    fn flags(&self) -> &shared_settings::Flags {
        &self.flags
    }

    fn isa_flags(&self) -> Vec<shared_settings::Value> {
        self.isa_flags.iter().collect()
    }

    fn isa_flags_hash_key(&self) -> IsaFlagsHashKey<'_> {
        IsaFlagsHashKey(self.isa_flags.hash_key())
    }

    #[cfg(feature = "unwind")]
    fn emit_unwind_info(
        &self,
        result: &CompiledCode,
        kind: crate::isa::unwind::UnwindInfoKind,
    ) -> CodegenResult<Option<crate::isa::unwind::UnwindInfo>> {
        use crate::isa::unwind::UnwindInfo;
        use crate::isa::unwind::UnwindInfoKind;
        Ok(match kind {
            // TODO: LoongArch unwinding
            // UnwindInfoKind::SystemV => {}
            UnwindInfoKind::Windows => None,
            _ => None,
        })
    }

    #[cfg(feature = "unwind")]
    fn create_systemv_cie(&self) -> Option<gimli::write::CommonInformationEntry> {
        // Some(inst::unwind::systemv::create_cie())
        // TODO: LoongArch unwinding
        None
    }

    fn text_section_builder(&self, num_funcs: usize) -> Box<dyn TextSectionBuilder> {
        Box::new(MachTextSectionBuilder::<inst::MInst>::new(num_funcs))
    }

    #[cfg(feature = "unwind")]
    fn map_regalloc_reg_to_dwarf(&self, reg: Reg) -> Result<u16, systemv::RegisterMappingError> {
        // inst::unwind::systemv::map_reg(reg).map(|reg| reg.0)
        unimplemented!();
    }

    fn function_alignment(&self) -> FunctionAlignment {
        inst::MInst::function_alignment()
    }

    fn page_size_align_log2(&self) -> u8 {
        debug_assert_eq!(1 << 12, 0x1000);
        12
    }

    #[cfg(feature = "disas")]
    fn to_capstone(&self) -> Result<capstone::Capstone, capstone::Error> {
        // TODO: capstone support
        Err(capstone::Error::UnsupportedArch)
    }

    fn pretty_print_reg(&self, reg: Reg, _size: u8) -> String {
        // TODO-LA: implement proper register pretty-printing.
        format!("{reg:?}")
    }

    fn has_native_fma(&self) -> bool {
        true
    }

    fn has_round(&self) -> bool {
        true
    }

    fn has_blendv_lowering(&self, _: Type) -> bool {
        false
    }

    fn has_x86_pshufb_lowering(&self) -> bool {
        false
    }

    fn has_x86_pmulhrsw_lowering(&self) -> bool {
        false
    }

    fn has_x86_pmaddubsw_lowering(&self) -> bool {
        false
    }

    fn default_argument_extension(&self) -> ir::ArgumentExtension {
        ir::ArgumentExtension::Sext
    }
}

impl fmt::Display for LoongArchBackend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MachBackend")
            .field("name", &self.name())
            .field("triple", &self.triple())
            .field("flags", &format!("{}", self.flags()))
            .finish()
    }
}

/// Create a new `isa::Builder`.
pub fn isa_builder(triple: Triple) -> IsaBuilder {
    match triple.architecture {
        Architecture::LoongArch64 => {}
        _ => unreachable!(),
    }
    IsaBuilder {
        triple,
        setup: la_settings::builder(),
        constructor: isa_constructor,
    }
}

fn isa_constructor(
    triple: Triple,
    shared_flags: Flags,
    builder: &shared_settings::Builder,
) -> CodegenResult<OwnedTargetIsa> {
    let isa_flags = la_settings::Flags::new(&shared_flags, builder);

    if !(isa_flags.has_32s() && isa_flags.has_64bit()) {
        // TODO: LA32R and LA32S support
        return Err(CodegenError::Unsupported(
            "The LoongArch Backend currently only supports 64-bit targets".into(),
        ));
    }

    let backend = LoongArchBackend::new_with_flags(triple, shared_flags, isa_flags);
    Ok(backend.wrapped())
}
