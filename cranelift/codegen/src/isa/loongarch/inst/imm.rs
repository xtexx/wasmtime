use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Copy, Default)]
pub struct Simm {
    bits: u32,
}

impl Simm {
    pub(crate) const ZERO: Self = Self { bits: 0 };

    #[inline]
    pub fn from_i32(bits: i32) -> Self {
        Self {
            bits: (bits as u32),
        }
    }

    #[inline]
    pub fn as_i32(&self) -> i32 {
        self.bits as i32
    }

    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}

impl Debug for Simm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_i32())
    }
}

impl Display for Simm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bits)
    }
}

impl<N: Into<i32>> From<N> for Simm {
    fn from(value: N) -> Self {
        Self::from_i32(value.into())
    }
}

#[derive(Clone, Copy, Default)]
pub struct Uimm {
    bits: u32,
}

impl Uimm {
    pub(crate) const ZERO: Self = Self { bits: 0 };

    #[inline]
    pub fn from_u32(bits: u32) -> Self {
        Self { bits }
    }

    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}

impl Debug for Uimm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bits)
    }
}

impl Display for Uimm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bits)
    }
}

impl<N: Into<u32>> From<N> for Uimm {
    fn from(value: N) -> Self {
        Self::from_u32(value.into())
    }
}
