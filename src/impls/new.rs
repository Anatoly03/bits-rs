use crate::structs::nums::*;

pub trait New {
    fn new() -> Self;
}

impl New for UBits {
    fn new() -> Self {
        UBits(Vec::new())
    }
}

impl New for BigInt {
    fn new() -> Self {
        BigInt(false, UBits::new())
    }
}