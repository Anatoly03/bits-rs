use std::cmp::*;
use std::ops::Add;

use crate::structs::nums::*;

impl Add for UBigInt {
    type Output = Self;

    /**
     * Consider you have the following [u4] arrays.
     * `[0110, 1111, 0001]`
     * `      [0000, 1111]`
     * The sum would be:
     * `[0111, 0000, 1111]`
     */
    fn add(self, rhs: Self) -> Self::Output {
        // Make sure "self" is the longer vector
        if self.0.len() < rhs.0.len() {
            return rhs + self;
        }

        let limit = max(self.0.len(), rhs.0.len());

        // Calculate the sum assuming "self" is the longer vector
        let mut result = Vec::new();
        let mut carry = 0;

        for i in 0..limit {
            let a1 = self.0.get(i).or(Some(&0)).unwrap();
            let a2 = self.0.get(i).or(Some(&0)).unwrap();

            result.push(a1 + a2 + carry);

            carry = if a1.checked_add(*a2).is_none() {1} else {0}
        }

        UBigInt(result)
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Add for UFraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
