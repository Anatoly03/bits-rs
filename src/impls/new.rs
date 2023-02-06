use crate::structs::nums::*;

pub trait New {
    fn new() -> Self;
}

impl New for UBits {
    fn new() -> Self {
        UBits(Vec::new())
    }
}

impl New for Bits {
    fn new() -> Self {
        Bits(false, UBits::new())
    }
}