//! LoongArch ISA: binary code emission.

use crate::ir::{self, LibCall, TrapCode};
use crate::isa::loongarch::inst::*;
use crate::isa::loongarch::lower::isle::generated_code::*;
use cranelift_control::ControlPlane;

pub struct EmitInfo {
    #[expect(dead_code, reason = "may want to be used in the future")]
    shared_flag: settings::Flags,
    isa_flags: super::super::la_settings::Flags,
}

impl EmitInfo {
    pub(crate) fn new(
        shared_flag: settings::Flags,
        isa_flags: super::super::la_settings::Flags,
    ) -> Self {
        Self {
            shared_flag,
            isa_flags,
        }
    }
}

pub(crate) fn reg_to_num(m: Reg) -> u32 {
    #[cfg(debug_assertions)]
    if m.to_real_reg().is_none() {
        return 0xff;
    }
    u32::from(m.to_real_reg().unwrap().hw_enc() & 31)
}

/// State carried between emissions of a sequence of instructions.
#[derive(Default, Clone, Debug)]
pub struct EmitState {
    /// The user stack map for the upcoming instruction, as provided to
    /// `pre_safepoint()`.
    user_stack_map: Option<ir::UserStackMap>,

    /// Only used during fuzz-testing. Otherwise, it is a zero-sized struct and
    /// optimized away at compiletime. See [cranelift_control].
    ctrl_plane: ControlPlane,

    frame_layout: FrameLayout,
}

impl EmitState {
    fn take_stack_map(&mut self) -> Option<ir::UserStackMap> {
        self.user_stack_map.take()
    }
}

impl MachInstEmitState<MInst> for EmitState {
    fn new(
        abi: &Callee<crate::isa::loongarch::abi::LoongArchMachineDeps>,
        ctrl_plane: ControlPlane,
    ) -> Self {
        EmitState {
            user_stack_map: None,
            ctrl_plane,
            frame_layout: abi.frame_layout().clone(),
        }
    }

    fn pre_safepoint(&mut self, user_stack_map: Option<ir::UserStackMap>) {
        self.user_stack_map = user_stack_map;
    }

    fn ctrl_plane_mut(&mut self) -> &mut ControlPlane {
        &mut self.ctrl_plane
    }

    fn take_ctrl_plane(self) -> ControlPlane {
        self.ctrl_plane
    }

    fn on_new_block(&mut self) {}

    fn frame_layout(&self) -> &FrameLayout {
        &self.frame_layout
    }
}

impl MInst {}

impl MachInstEmit for MInst {
    type State = EmitState;
    type Info = EmitInfo;

    fn emit(&self, sink: &mut MachBuffer<MInst>, emit_info: &Self::Info, state: &mut EmitState) {
        // N.B.: we *must* not exceed the "worst-case size" used to compute
        // where to insert islands, except when islands are explicitly triggered
        // (with an `EmitIsland`). We check this in debug builds. This is `mut`
        // to allow disabling the check for `JTSequence`, which is always
        // emitted following an `EmitIsland`.
        let mut start_off = sink.cur_offset();

        match self {
            MInst::Nop0 => {}
            MInst::Nop4 => {
                sink.put4(encode(&LInst::enc_or(
                    writable_zero_reg(),
                    zero_reg(),
                    zero_reg(),
                )));
            }
            MInst::LInst { inst } => {
                sink.put4(encode(inst));
            }
            MInst::LoadImmGpr { dst, imm } => {
                if is_uimm_fit(*imm, 12) {
                    sink.put4(encode(&LInst::enc_ori(*dst, zero_reg(), *imm as u32)));
                } else {
                    // TODO: optimize load with only LU52I
                    let imm = *imm;
                    let high32 = imm >> 32;
                    let highest12 = (imm >> 52) & 0xFFF;
                    let highest13 = (imm >> 51) & 0x1FFF;
                    let higher20 = high32 & 0xFFFFF;
                    let higher21 = (imm >> 31) & 0x1FFFFF;
                    let low32 = imm & 0xFFFFFFFF;
                    let high20 = (imm >> 12) & 0xFFFFF;
                    let high21 = low32 >> 11;
                    let low12 = imm & 0xFFF;

                    // low32
                    if high20 == 0 {
                        sink.put4(encode(&LInst::enc_ori(*dst, zero_reg(), low12 as u32)));
                    } else if matches!(high21.count_ones(), 0 | 21) {
                        sink.put4(encode(&LInst::enc_addi_w(*dst, zero_reg(), low32 as i32)));
                    } else {
                        sink.put4(encode(&LInst::enc_lu12i_w(*dst, high20 as u32 as i32)));
                        if low12 != 0 {
                            sink.put4(encode(&LInst::enc_ori(*dst, dst.to_reg(), low12 as u32)));
                        }
                    }

                    // high32
                    if !matches!(higher21.count_ones(), 0 | 21) {
                        sink.put4(encode(&LInst::enc_lu32i_d(*dst, higher20 as i32)));
                    }
                    if !matches!(highest13.count_ones(), 0 | 13) {
                        sink.put4(encode(&LInst::enc_lu52i_d(
                            *dst,
                            dst.to_reg(),
                            highest12 as i32,
                        )));
                    }
                }
            }
            MInst::EmitIsland { needed_space } => {
                if sink.island_needed(*needed_space) {
                    let jump_around_label = sink.get_label();
                    MInst::gen_jump(jump_around_label).emit(sink, emit_info, state);
                    sink.emit_island(needed_space + 4, &mut state.ctrl_plane);
                    sink.bind_label(jump_around_label, &mut state.ctrl_plane);
                }
            }
            MInst::DummyUse { .. } => {}
            MInst::Move { rd, rj, ty } => {
                let rd = *rd;
                let rj = *rj;
                match (rd.to_reg().class(), rj.class()) {
                    (RegClass::Int, RegClass::Int) => {
                        sink.put4(encode(&LInst::enc_or(rd, rj, zero_reg())))
                    }
                    (RegClass::Int, RegClass::Float) => {
                        sink.put4(encode(&LInst::enc_or(rd, rj, zero_reg())))
                    }
                    (RegClass::Int, RegClass::Vector) => todo!(),
                    (RegClass::Float, RegClass::Int) if *ty == F32 && emit_info.isa_flags.has_f() => {
                        sink.put4(encode(&LInst::enc_movgr2fr_w(rd, rj)))
                    },
                    (RegClass::Float, RegClass::Int) if *ty == F64 && emit_info.isa_flags.has_d() => {
                        sink.put4(encode(&LInst::enc_movgr2fr_d(rd, rj)))
                    },
                    (RegClass::Float, RegClass::Int) => todo!(),
                    (RegClass::Float, RegClass::Float) if *ty == F32 && emit_info.isa_flags.has_f() => {
                        sink.put4(encode(&LInst::enc_fmov_s(rd, rj)))
                    },
                    (RegClass::Float, RegClass::Float) if *ty == F64 && emit_info.isa_flags.has_d() => {
                        sink.put4(encode(&LInst::enc_fmov_d(rd, rj)))
                    },
                    (RegClass::Float, RegClass::Float) => todo!(),
                    (RegClass::Float, RegClass::Vector) => unreachable!(),
                    (RegClass::Vector, RegClass::Int) => todo!(),
                    (RegClass::Vector, RegClass::Vector) => todo!(),
                    (RegClass::Vector, RegClass::Float) => unreachable!(),
                }
            }
            MInst::Trap => {
                sink.put4(encode(&LInst::enc_break(0)));
            }
            MInst::Call { info } => {
                let start = sink.cur_offset();

                sink.add_reloc(Reloc::LoongArchCall36, &info.dest, 0);
                sink.put4(encode(&LInst::enc_pcaddu18i(writable_ra_reg(), 0)));
                sink.put4(encode(&LInst::enc_jirl(writable_ra_reg(), ra_reg(), 0)));

                if let Some(s) = state.take_stack_map() {
                    let offset = sink.cur_offset();
                    sink.push_user_stack_map(state, offset, s);
                }

                if let Some(try_call) = info.try_call_info.as_ref() {
                    sink.add_try_call_site(
                        Some(state.frame_layout.sp_to_fp()),
                        try_call.exception_handlers(&state.frame_layout),
                    );
                } else {
                    sink.add_call_site();
                }

                let callee_pop_size = i32::try_from(info.callee_pop_size).unwrap();
                if callee_pop_size > 0 {
                    for inst in LoongArchMachineDeps::gen_sp_reg_adjust(-callee_pop_size) {
                        inst.emit(sink, emit_info, state);
                    }
                }

                if info.patchable {
                    sink.add_patchable_call_site(sink.cur_offset() - start);
                } else {
                    // Load any stack-carried return values.
                    info.emit_retval_loads::<LoongArchMachineDeps, _, _>(
                        state.frame_layout().stackslots_size,
                        |inst| inst.emit(sink, emit_info, state),
                        |needed_space| Some(MInst::EmitIsland { needed_space }),
                    );
                }

                // If this is a try-call, jump to the continuation
                // (normal-return) block.
                if let Some(try_call) = info.try_call_info.as_ref() {
                    MInst::Branch {
                        label: try_call.continuation,
                    }
                    .emit(sink, emit_info, state);
                }

                start_off = sink.cur_offset();
            }
            MInst::CallInd { info } => {
                sink.put4(encode(&LInst::enc_jirl(writable_ra_reg(), info.dest, 0)));

                if let Some(s) = state.take_stack_map() {
                    let offset = sink.cur_offset();
                    sink.push_user_stack_map(state, offset, s);
                }

                if let Some(try_call) = info.try_call_info.as_ref() {
                    sink.add_try_call_site(
                        Some(state.frame_layout.sp_to_fp()),
                        try_call.exception_handlers(&state.frame_layout),
                    );
                } else {
                    sink.add_call_site();
                }

                let callee_pop_size = i32::try_from(info.callee_pop_size).unwrap();
                if callee_pop_size > 0 {
                    for inst in LoongArchMachineDeps::gen_sp_reg_adjust(-callee_pop_size) {
                        inst.emit(sink, emit_info, state);
                    }
                }

                // Load any stack-carried return values.
                info.emit_retval_loads::<LoongArchMachineDeps, _, _>(
                    state.frame_layout().stackslots_size,
                    |inst| inst.emit(sink, emit_info, state),
                    |needed_space| Some(MInst::EmitIsland { needed_space }),
                );

                // If this is a try-call, jump to the continuation
                // (normal-return) block.
                if let Some(try_call) = info.try_call_info.as_ref() {
                    MInst::Branch {
                        label: try_call.continuation,
                    }
                    .emit(sink, emit_info, state);
                }

                start_off = sink.cur_offset();
            }
            MInst::ReturnCall { info } => todo!(),
            MInst::ReturnCallInd { info } => todo!(),
            MInst::Args { .. } | MInst::Rets { .. } => {
                // Ppseudo-instructions that serve
                // only to constrain registers at a certain point.
            }
            MInst::Ret => {
                sink.put4(encode(&LInst::enc_jirl(writable_ra_reg(), ra_reg(), 0)));
            }
            MInst::Branch { label } => {
                let label = *label;
                sink.use_label_at_offset(start_off, label, LabelUse::B26);
                sink.add_uncond_branch(start_off, start_off + 4, label);
                sink.put4(encode(&LInst::enc_b(0, 0)));
            }
            MInst::CondBr {
                taken,
                not_taken,
                kind,
            } => {
                match taken {
                    CondBrTarget::Label(label) => {
                        let code = kind.emit();
                        let code_inverse = kind.inverse().emit().to_le_bytes();
                        sink.use_label_at_offset(start_off, *label, LabelUse::B16);
                        sink.add_cond_branch(start_off, start_off + 4, *label, &code_inverse);
                        sink.put4(code);
                    }
                    CondBrTarget::Fallthrough => panic!("Cannot fallthrough in taken target"),
                }

                match not_taken {
                    CondBrTarget::Label(label) => {
                        MInst::gen_jump(*label).emit(sink, emit_info, state)
                    }
                    CondBrTarget::Fallthrough => {}
                };
            }
            MInst::Select {
                dst,
                condition,
                x,
                y,
            } => todo!(),
            MInst::BrTable {
                index,
                tmp1,
                tmp2,
                targets,
            } => todo!(),
            MInst::StackProbeLoop {
                guard_size,
                probe_count,
                tmp,
            } => todo!(),
            MInst::Load {
                rd,
                op,
                flags,
                from,
            } => {
                let base = from.get_base_register();
                let offset = from.get_offset_with_state(state);
                let offset_imm12 = is_simm_fit(offset, 12);
                let label = from.get_label_with_sink(sink);

                let (addr, offset) = match (base, offset_imm12, label) {
                    // When loading from a Reg+Offset, if the offset fits into an imm12 we can directly encode it.
                    (Some(base), true, None) => (base, offset as i32),

                    // Otherwise, if the offset does not fit into a imm12, we need to materialize it into a
                    // register and load from that.
                    (Some(_), false, None) => {
                        let tmp = writable_spilltmp_reg();
                        for inst in from.load_addr(tmp, state, sink) {
                            inst.emit(sink, emit_info, state);
                        }
                        (tmp.to_reg(), 0)
                    }

                    // If the AMode contains a label we can emit an internal relocation that gets
                    // resolved with the correct address later.
                    (None, true, Some(label)) => {
                        debug_assert_eq!(offset, 0);
                        todo!();
                    }

                    // These cases are impossible with the current AModes that we have. We either
                    // always have a register, or always have a label. Never both, and never neither.
                    (None, _, None) | (Some(_), _, Some(_)) | (None, false, Some(_)) => {
                        unreachable!("Invalid load address")
                    }
                };

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual load instruction starts.
                    sink.add_trap(trap_code);
                }

                let offset = Simm::from(offset);
                sink.put4(encode(&LInst::DmJSk12 {
                    op: match *op {
                        LoadOP::Lb => DmJSk12Op::LD_B,
                        LoadOP::Lh => DmJSk12Op::LD_H,
                        LoadOP::Lw => DmJSk12Op::LD_W,
                        LoadOP::Lbu => DmJSk12Op::LD_BU,
                        LoadOP::Lhu => DmJSk12Op::LD_HU,
                        LoadOP::Lwu => DmJSk12Op::LD_WU,
                        LoadOP::Ld => DmJSk12Op::LD_D,
                        LoadOP::Flh => todo!(),
                        LoadOP::Flw => todo!(),
                        LoadOP::Fld => todo!(),
                    },
                    rd: *rd,
                    rj: addr,
                    imm1: offset,
                }));
            }
            MInst::Store { to, op, flags, src } => {
                let base = to.get_base_register();
                let offset = to.get_offset_with_state(state);
                let offset_imm12 = is_simm_fit(offset, 12);

                let (addr, offset) = match (base, offset_imm12) {
                    // If the offset fits into an imm12 we can directly encode it.
                    (Some(base), true) => (base, offset as i32),
                    // Otherwise load the address it into a reg and load from it.
                    _ => {
                        let tmp = writable_spilltmp_reg();
                        for inst in to.load_addr(tmp, state, sink) {
                            inst.emit(sink, emit_info, state);
                        }
                        (tmp.to_reg(), 0)
                    }
                };

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual load instruction starts.
                    sink.add_trap(trap_code);
                }

                let offset = Simm::from(offset);
                sink.put4(encode(&LInst::DJSk12 {
                    op: match *op {
                        StoreOP::Sb => DJSk12Op::ST_B,
                        StoreOP::Sh => DJSk12Op::ST_H,
                        StoreOP::Sw => DJSk12Op::ST_W,
                        StoreOP::Sd => DJSk12Op::ST_D,
                        StoreOP::Fsh => todo!(),
                        StoreOP::Fsw => todo!(),
                        StoreOP::Fsd => todo!(),
                    },
                    rd: *src,
                    rj: addr,
                    imm1: offset,
                }));
            }
            MInst::LoadExtNameGot { rd, name } => todo!(),
            MInst::LoadExtNameNear { rd, name, offset } => {
                // "medium" code model
                // Compared with "normal", "medium" saves one instruction here and
                // addresses a much larger space.
                sink.add_reloc(Reloc::LoongArchCall36, &**name, *offset);
                sink.put4(encode(&LInst::enc_pcaddu18i(*rd, 0)));
            }
            MInst::LoadExtNameFar {
                rd,
                name,
                offset,
                tmp_reg,
            } => {
                // "extreme" code model
                sink.add_reloc(Reloc::LoongArchPcalaHi20, &**name, *offset);
                sink.put4(encode(&LInst::enc_pcalau12i(*tmp_reg, 0)));
                sink.add_reloc(Reloc::LoongArchPcalaLo12, &**name, *offset);
                sink.put4(encode(&LInst::enc_addi_d(*rd, zero_reg(), 0)));
                sink.add_reloc(Reloc::LoongArchPcala64Lo20, &**name, *offset);
                sink.put4(encode(&LInst::enc_lu32i_d(*rd, 0)));
                sink.add_reloc(Reloc::LoongArchPcala64Hi12, &**name, *offset);
                sink.put4(encode(&LInst::enc_lu52i_d(*rd, rd.to_reg(), 0)));
                sink.put4(encode(&LInst::enc_add_d(
                    *rd,
                    rd.to_reg(),
                    tmp_reg.to_reg(),
                )));
            }
        }

        // We exclude br_table, call, return_call and try_call from
        // these checks since they emit their own islands, and thus
        // are allowed to exceed the worst case size.
        let emits_own_island = match self {
            MInst::BrTable { .. }
            | MInst::ReturnCall { .. }
            | MInst::ReturnCallInd { .. }
            | MInst::Call { .. }
            | MInst::CallInd { .. }
            | MInst::EmitIsland { .. } => true,
            _ => false,
        };
        if !emits_own_island {
            let end_off = sink.cur_offset();
            assert!(
                (end_off - start_off) <= MInst::worst_case_size(),
                "MInst:{:?} length:{} worst_case_size:{}",
                self,
                end_off - start_off,
                MInst::worst_case_size()
            );
        }
    }

    fn pretty_print_inst(&self, state: &mut Self::State) -> String {
        self.print_with_state(state)
    }
}
