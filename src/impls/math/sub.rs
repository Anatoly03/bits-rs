use std::ops::Sub;

use crate::structs::nums::*;

/**
 * UBigInt + UBigInt
 */
impl<'a, 'b> Sub<&'b UBits> for &'a UBits {
    type Output = UBits;

    fn sub(self, rhs: &'b UBits) -> Self::Output {
        todo!()

        //trim(self)
    }
}

/**
 * BigInt + BigInt
 */
impl<'a, 'b> Sub<&'b Bits> for &'a Bits {
    type Output = Bits;

    fn sub(self, rhs: &'b Bits) -> Self::Output {
        todo!()

        //trim(self.1)
        //set_sign_for_0(self)
    }
}
