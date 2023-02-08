use crate::structs::{nums::*, consts::Atomic};

impl From<Vec<Atomic>> for UBits {
    fn from(v: Vec<Atomic>) -> Self {
        UBits(v)
    }
}

impl From<Vec<Atomic>> for Bits {
    fn from(v: Vec<Atomic>) -> Self {
        Bits(false, UBits::from(v))
    }
}