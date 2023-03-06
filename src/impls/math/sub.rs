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
