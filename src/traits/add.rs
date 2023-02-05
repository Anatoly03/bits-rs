use std::ops::Add;

use crate::structs::*;

impl Add for UBigInt {
    type Output = UBigInt;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Add for UFraction {
    type Output = UFraction;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}