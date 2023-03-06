use std::ops::Sub;

use crate::structs::nums::*;

/**
 * UBigInt + UBigInt
 */
impl<'a, 'b> Sub<&'b Bits> for &'a Bits {
    type Output = Bits;

    fn sub(self, rhs: &'b Bits) -> Self::Output {
        todo!()

        //trim(self)
    }
}
