use crate::structs::{consts::Atomic, nums::*};

impl From<Vec<Atomic>> for UBits {
    fn from(v: Vec<Atomic>) -> Self {
        UBits(v)
    }
}
