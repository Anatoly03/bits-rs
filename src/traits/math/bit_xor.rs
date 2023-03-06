use std::cmp::*;
use std::ops::BitXor;

use crate::structs::nums::*;

/**
 * UBigInt + UBigInt
 */
impl<'a, 'b> BitXor<&'b Bits> for &'a Bits {
    type Output = Bits;

    fn bitxor(self, rhs: &'b Bits) -> Self::Output {
        let mut result = Vec::new();
        let limit = max(self.0.len(), rhs.0.len());

        for i in 0..limit {
            let a1 = self.0.get(i).or(Some(&0)).unwrap();
            let a2 = rhs.0.get(i).or(Some(&0)).unwrap();

            result.push(a1 ^ a2);
        }

        Bits(result)
    }
}
