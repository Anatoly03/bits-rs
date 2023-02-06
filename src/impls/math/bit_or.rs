use std::cmp::*;
use std::ops::BitOr;

use crate::structs::nums::*;

/**
 * UBigInt + UBigInt
 */
impl<'a, 'b> BitOr<&'b UBits> for &'a UBits {
    type Output = UBits;

    fn bitor(self, rhs: &'b UBits) -> Self::Output {
        let mut result = Vec::new();
        let limit = max(self.0.len(), rhs.0.len());

        for i in 0..limit {
            let a1 = self.0.get(i).or(Some(&0)).unwrap();
            let a2 = rhs.0.get(i).or(Some(&0)).unwrap();

            result.push(a1 | a2);
        }

        UBits(result)
    }
}

/**
 * BigInt + BigInt
 */
impl<'a, 'b> BitOr<&'b BigInt> for &'a BigInt {
    type Output = BigInt;

    fn bitor(self, rhs: &'b BigInt) -> Self::Output {
        BigInt(self.0 == rhs.0, &self.1 | &rhs.1)
    }
}
