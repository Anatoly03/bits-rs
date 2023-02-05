
use std::ops::Neg;

use crate::structs::nums::*;

impl Neg for UBigInt {
    type Output = BigInt;

    fn neg(self) -> Self::Output {
        BigInt(true, self)
    }
}

impl Neg for BigInt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        BigInt(!self.0, self.1)
    }
}

impl Neg for UFraction {
    type Output = Fraction;

    fn neg(self) -> Self::Output {
        Fraction(true, self)
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Fraction(!self.0, self.1)
    }
}