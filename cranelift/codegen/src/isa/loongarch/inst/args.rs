//! LoongArch ISA definitions: instruction arguments.

use super::*;
use crate::ir::condcodes::CondCode;
use crate::isa::loongarch::lower::isle::generated_code::*;
use crate::machinst::isle::WritableReg;

use core::fmt::Result;

/// An addressing mode specified for a load/store operation.
#[derive(Clone, Debug, Copy)]
pub enum AMode {
    /// Arbitrary offset from a register. Converted to generation of large
    /// offsets with multiple instructions as necessary during code emission.
    RegOffset(Reg, i64),
    /// Offset from the stack pointer.
    SPOffset(i64),
    /// Offset from the frame pointer.
    FPOffset(i64),

    /// Offset into the slot area of the stack, which lies just above the
    /// outgoing argument area that's setup by the function prologue.
    /// At emission time, this is converted to `SPOffset` with a fixup added to
    /// the offset constant. The fixup is a running value that is tracked as
    /// emission iterates through instructions in linear order, and can be
    /// adjusted up and down with [MInst::VirtualSPOffsetAdj].
    ///
    /// The standard ABI is in charge of handling this (by emitting the
    /// adjustment meta-instructions). See the diagram in the documentation
    /// for [crate::isa::aarch64::abi](the ABI module) for more details.
    SlotOffset(i64),

    /// Offset into the argument area.
    IncomingArg(i64),

    /// A reference to a constant which is placed outside of the function's
    /// body, typically at the end.
    Const(VCodeConstant),

    /// A reference to a label.
    Label(MachLabel),
}

impl AMode {
    /// Add the registers referenced by this AMode to `collector`.
    pub(crate) fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            AMode::RegOffset(reg, ..) => collector.reg_use(reg),
            // Registers used in these modes aren't allocatable.
            AMode::SPOffset(..)
            | AMode::FPOffset(..)
            | AMode::SlotOffset(..)
            | AMode::IncomingArg(..)
            | AMode::Const(..)
            | AMode::Label(..) => {}
        }
    }

    pub(crate) fn get_base_register(&self) -> Option<Reg> {
        match self {
            &AMode::RegOffset(reg, ..) => Some(reg),
            &AMode::SPOffset(..) => Some(stack_reg()),
            &AMode::FPOffset(..) => Some(fp_reg()),
            &AMode::SlotOffset(..) => Some(stack_reg()),
            &AMode::IncomingArg(..) => Some(stack_reg()),
            &AMode::Const(..) | AMode::Label(..) => None,
        }
    }

    pub(crate) fn get_offset_with_state(&self, state: &EmitState) -> i64 {
        match self {
            &AMode::SlotOffset(offset) => {
                offset + i64::from(state.frame_layout().outgoing_args_size)
            }

            // Compute the offset into the incoming argument area relative to SP
            &AMode::IncomingArg(offset) => {
                let frame_layout = state.frame_layout();
                let sp_offset = frame_layout.tail_args_size
                    + frame_layout.setup_area_size
                    + frame_layout.clobber_size
                    + frame_layout.fixed_frame_storage_size
                    + frame_layout.outgoing_args_size;
                i64::from(sp_offset) - offset
            }

            &AMode::RegOffset(_, offset) => offset,
            &AMode::SPOffset(offset) => offset,
            &AMode::FPOffset(offset) => offset,
            &AMode::Const(_) | &AMode::Label(_) => 0,
        }
    }

    /// Retrieve a MachLabel that corresponds to this addressing mode, if it exists.
    pub(crate) fn get_label_with_sink(&self, sink: &mut MachBuffer<MInst>) -> Option<MachLabel> {
        match self {
            &AMode::Const(addr) => Some(sink.get_label_for_constant(addr)),
            &AMode::Label(label) => Some(label),
            &AMode::RegOffset(..)
            | &AMode::SPOffset(..)
            | &AMode::FPOffset(..)
            | &AMode::IncomingArg(..)
            | &AMode::SlotOffset(..) => None,
        }
    }

    pub(crate) fn load_addr(
        &self,
        dst: WritableReg,
        state: &EmitState,
        sink: &mut MachBuffer<MInst>,
    ) -> SmallInstVec<MInst> {
        let base = self.get_base_register();
        let offset = self.get_offset_with_state(state);
        let offset_imm12 = is_simm_fit(offset, 12);

        // TODO: LA32
        match (self, base, offset_imm12) {
            (_, Some(rj), true) => {
                let mut insts = SmallInstVec::new();
                insts.push(LInst::enc_addi_d(dst, rj, offset as i32).into());
                insts
            }
            (_, Some(rj), false) => {
                let mut insts = SmallInstVec::new();
                insts.push(MInst::load_imm(dst, offset as u64));
                insts.push(LInst::enc_add_d(dst, dst.to_reg(), rj).into());
                insts
            }
            (AMode::Const(addr), None, _) => {
                // Get an address label for the constant and recurse.
                let label = sink.get_label_for_constant(*addr);
                AMode::Label(label).load_addr(dst, state, sink)
            }
            (AMode::Label(label), None, _) => {
                todo!();
            }
            (amode, _, _) => {
                unimplemented!("LoadAddr: {:?}", amode);
            }
        }
    }
}

impl Display for AMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AMode::RegOffset(r, offset, ..) => {
                write!(f, "{}({})", offset, format_reg(*r))
            }
            AMode::SPOffset(offset, ..) => {
                write!(f, "{offset}(sp)")
            }
            AMode::SlotOffset(offset, ..) => {
                write!(f, "{offset}(slot)")
            }
            AMode::IncomingArg(offset) => {
                write!(f, "-{offset}(incoming_arg)")
            }
            AMode::FPOffset(offset, ..) => {
                write!(f, "{offset}(fp)")
            }
            AMode::Const(addr, ..) => {
                write!(f, "[const({})]", addr.as_u32())
            }
            AMode::Label(label) => {
                write!(f, "[label{}]", label.as_u32())
            }
        }
    }
}

impl From<StackAMode> for AMode {
    fn from(stack: StackAMode) -> AMode {
        match stack {
            StackAMode::IncomingArg(offset, stack_args_size) => {
                AMode::IncomingArg(i64::from(stack_args_size) - offset)
            }
            StackAMode::OutgoingArg(offset) => AMode::SPOffset(offset),
            StackAMode::Slot(offset) => AMode::SlotOffset(offset),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct IntegerCompare {
    pub(crate) kind: IntCC,
    pub(crate) rs1: Reg,
    pub(crate) rs2: Reg,
}

impl IntegerCompare {
    pub(crate) fn emit(self) -> u32 {
        encode(&match self.kind {
            IntCC::Equal => LInst::enc_beq(self.rs1, self.rs2, 0),
            IntCC::NotEqual => LInst::enc_bne(self.rs1, self.rs2, 0),
            IntCC::SignedLessThan => LInst::enc_blt(self.rs1, self.rs2, 0),
            IntCC::SignedGreaterThanOrEqual => LInst::enc_bge(self.rs1, self.rs2, 0),
            IntCC::SignedGreaterThan => LInst::enc_blt(self.rs2, self.rs1, 0),
            IntCC::SignedLessThanOrEqual => LInst::enc_bge(self.rs2, self.rs1, 0),
            IntCC::UnsignedLessThan => LInst::enc_bltu(self.rs1, self.rs2, 0),
            IntCC::UnsignedGreaterThanOrEqual => LInst::enc_bgeu(self.rs1, self.rs2, 0),
            IntCC::UnsignedGreaterThan => LInst::enc_bltu(self.rs2, self.rs1, 0),
            IntCC::UnsignedLessThanOrEqual => LInst::enc_bgeu(self.rs2, self.rs1, 0),
        })
    }

    pub(crate) fn inverse(self) -> Self {
        Self {
            kind: self.kind.complement(),
            ..self
        }
    }

    pub(crate) fn regs(&self) -> [Reg; 2] {
        [self.rs1, self.rs2]
    }
}

impl LoadOP {
    pub(crate) fn op_name(self) -> &'static str {
        match self {
            Self::Lb => "ld.b",
            Self::Lh => "ld.h",
            Self::Lw => "ld.w",
            Self::Lbu => "ld.bu",
            Self::Lhu => "ld.hu",
            Self::Lwu => "ld.wu",
            Self::Ld => "ld.d",
            Self::Flh => "fld.h",
            Self::Flw => "fld.w",
            Self::Fld => "fld.d",
        }
    }

    pub(crate) fn from_type(ty: Type) -> Self {
        match ty {
            F16 => Self::Flh,
            F32 => Self::Flw,
            F64 => Self::Fld,
            I8 => Self::Lb,
            I16 => Self::Lh,
            I32 => Self::Lw,
            I64 => Self::Ld,
            _ => unreachable!(),
        }
    }
}

impl StoreOP {
    pub(crate) fn op_name(self) -> &'static str {
        match self {
            Self::Sb => "st.b",
            Self::Sh => "st.h",
            Self::Sw => "st.w",
            Self::Sd => "st.d",
            Self::Fsh => "fst.h",
            Self::Fsw => "fst.w",
            Self::Fsd => "fst.d",
        }
    }
    pub(crate) fn from_type(ty: Type) -> Self {
        match ty {
            F16 => Self::Fsh,
            F32 => Self::Fsw,
            F64 => Self::Fsd,
            I8 => Self::Sb,
            I16 => Self::Sh,
            I32 => Self::Sw,
            I64 => Self::Sd,
            _ => unreachable!(),
        }
    }
}
