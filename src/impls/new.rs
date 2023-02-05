use crate::structs::nums::*;

pub trait New {
    fn new() -> Self;
}

impl New for UBigInt {
    fn new() -> Self {
        UBigInt(Vec::new())
    }
}

impl New for BigInt {
    fn new() -> Self {
        BigInt(false, UBigInt::new())
    }
}