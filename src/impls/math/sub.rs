use std::ops::Sub;

use crate::structs::nums::*;

/**
 * UBigInt + UBigInt
 */
impl<'a, 'b> Sub<&'b UBigInt> for &'a UBigInt {
    type Output = UBigInt;

    fn sub(self, rhs: &'b UBigInt) -> Self::Output {
        todo!()
    }
}

/**
 * BigInt + BigInt
 */
impl<'a, 'b> Sub<&'b BigInt> for &'a BigInt {
    type Output = BigInt;

    fn sub(self, rhs: &'b BigInt) -> Self::Output {
        todo!()
    }
}
