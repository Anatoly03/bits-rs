use crate::structs::{nums::*, consts::Atomic};

impl UBits {
    pub fn from_vec(v: Vec<Atomic>) -> Self {
        UBits(v)
    }
}

impl Bits {
    pub fn from_vec(v: Vec<Atomic>) -> Self {
        Bits(false, UBits::from_vec(v))
    }
}