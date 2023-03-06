use crate::structs::{consts::Atomic, nums::*};

impl From<Vec<Atomic>> for Bits {
    fn from(v: Vec<Atomic>) -> Self {
        Bits(v)
    }
}
