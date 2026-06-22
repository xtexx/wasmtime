//! LoongArch ISA definitions: registers.
//!

use crate::machinst::{Reg, Writable};

use regalloc2::{PReg, RegClass, VReg};

#[inline]
pub const fn a0() -> Reg {
    x_reg(6)
}

#[inline]
pub const fn a1() -> Reg {
    x_reg(7)
}

#[inline]
pub fn writable_a0() -> Writable<Reg> {
    Writable::from_reg(a0())
}
#[inline]
#[cfg(test)]
pub fn writable_a1() -> Writable<Reg> {
    Writable::from_reg(a1())
}

#[inline]
#[cfg(test)]
pub fn fa0() -> Reg {
    f_reg(0)
}
#[inline]
#[cfg(test)]
pub fn writable_fa0() -> Writable<Reg> {
    Writable::from_reg(fa0())
}
#[inline]
pub fn fa1() -> Reg {
    f_reg(1)
}
#[inline]
#[expect(dead_code, reason = "here if needed in the future")]
pub fn writable_fa1() -> Writable<Reg> {
    Writable::from_reg(fa1())
}

/// Get a reference to the zero-register.
#[inline]
pub fn zero_reg() -> Reg {
    x_reg(0)
}

/// Get a writable reference to the zero-register (this discards a result).
#[inline]
pub fn writable_zero_reg() -> Writable<Reg> {
    Writable::from_reg(zero_reg())
}

#[inline]
pub fn stack_reg() -> Reg {
    x_reg(3)
}

#[inline]
pub fn writable_stack_reg() -> Writable<Reg> {
    Writable::from_reg(stack_reg())
}

pub fn ra_reg() -> Reg {
    x_reg(1)
}

#[inline]
pub fn writable_ra_reg() -> Writable<Reg> {
    Writable::from_reg(ra_reg())
}

#[inline]
pub fn fp_reg() -> Reg {
    x_reg(22)
}

#[inline]
pub fn writable_fp_reg() -> Writable<Reg> {
    Writable::from_reg(fp_reg())
}

#[inline]
pub const fn x_reg(enc: usize) -> Reg {
    let p_reg = PReg::new(enc, RegClass::Int);
    let v_reg = VReg::new(p_reg.index(), p_reg.class());
    Reg::from_virtual_reg(v_reg)
}
pub const fn px_reg(enc: usize) -> PReg {
    PReg::new(enc, RegClass::Int)
}

#[inline]
pub fn f_reg(enc: usize) -> Reg {
    let p_reg = PReg::new(enc, RegClass::Float);
    let v_reg = VReg::new(p_reg.index(), p_reg.class());
    Reg::from(v_reg)
}
pub const fn pf_reg(enc: usize) -> PReg {
    PReg::new(enc, RegClass::Float)
}

#[inline]
pub fn fcc_reg(enc: usize) -> Reg {
    let p_reg = PReg::new(enc, RegClass::Vector);
    let v_reg = VReg::new(p_reg.index(), p_reg.class());
    Reg::from(v_reg)
}
pub const fn pfcc_reg(enc: usize) -> PReg {
    PReg::new(enc, RegClass::Vector)
}

pub fn spilltmp_reg() -> Reg {
    x_reg(12)
}

/// Get a writable reference to the spilltmp reg.
pub fn writable_spilltmp_reg() -> Writable<Reg> {
    Writable::from_reg(spilltmp_reg())
}
