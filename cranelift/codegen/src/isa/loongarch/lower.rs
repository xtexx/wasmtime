//! Lowering rules for LoongArch.
use crate::ir::Inst as IRInst;
use crate::isa::loongarch::LoongArchBackend;
use crate::isa::loongarch::inst::*;
use crate::machinst::lower::*;
use crate::machinst::*;
pub mod isle;

//=============================================================================
// Lowering-backend trait implementation.

impl LowerBackend for LoongArchBackend {
    type MInst = MInst;

    fn lower(&self, ctx: &mut Lower<MInst>, ir_inst: IRInst) -> Option<InstOutput> {
        isle::lower(ctx, self, ir_inst)
    }

    fn lower_branch(
        &self,
        ctx: &mut Lower<MInst>,
        ir_inst: IRInst,
        targets: &[MachLabel],
    ) -> Option<()> {
        isle::lower_branch(ctx, self, ir_inst, targets)
    }

    fn maybe_pinned_reg(&self) -> Option<Reg> {
        None
    }
}
