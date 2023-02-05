use std::cmp::*;

use crate::structs::nums::*;

/**
 * UBigInt + UBigInt
 */
impl PartialOrd for UBigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let limit = max(self.0.len(), other.0.len());

        for i in 0..limit {
            let s = self.0.get(i).or(Some(&0)).unwrap();
            let o = other.0.get(i).or(Some(&0)).unwrap();

            if s < o {
                return Some(Ordering::Less);
            } else if s < o {
                return Some(Ordering::Greater);
            }
        }

        Some(Ordering::Equal)
    }
}

/**
 * BigInt + BigInt
 */
impl PartialOrd for BigInt {
    // TODO fix bug where -0 and +0 are not the same
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.0, other.0) {
            (true, true) => other.1.partial_cmp(&self.1), // negative compare
            (true, false) => Some(Ordering::Less),        // self is negative
            (false, true) => Some(Ordering::Greater),     // other is negative
            (false, false) => self.1.partial_cmp(&other.1), // positive compare
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
