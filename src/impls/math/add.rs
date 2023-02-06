use std::cmp::*;
use std::ops::Add;

use crate::structs::nums::*;

/**
 * UBigInt + UBigInt
 */
impl<'a, 'b> Add<&'b UBits> for &'a UBits {
    type Output = UBits;

    /**
     * Consider you have the following [u4] arrays.
     * `[0110, 1111, 0001]`
     * `      [0000, 1111]`
     * The sum would be:
     * `[0111, 0000, 1111]`
     */
    fn add(self, rhs: &'b UBits) -> Self::Output {
        let limit = max(self.0.len(), rhs.0.len());

        let mut result = Vec::new();
        let mut carry = 0;

        for i in 0..limit {
            let a1 = self.0.get(i).or(Some(&0)).unwrap();
            let a2 = rhs.0.get(i).or(Some(&0)).unwrap();
            let sum = a1.wrapping_add(*a2).wrapping_add(carry);

            result.push(sum);

            carry = if a1.checked_add(*a2).is_none() { 1 } else { 0 }
        }

        UBits(result)
    }
}

/**
 * BigInt + BigInt
 */
impl<'a, 'b> Add<&'b Bits> for &'a Bits {
    type Output = Bits;

    fn add(self, rhs: &'b Bits) -> Self::Output {
        if self.0 == rhs.0 {
            Bits(self.0, self.1.add(&rhs.1))
        } else {
            todo!()
        }
    }
}

// impl Add for UFraction {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         todo!()
//     }
// }

// impl Add for Fraction {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         todo!()
//     }
// }
