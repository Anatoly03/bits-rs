use std::cmp::*;

use crate::structs::nums::*;

/**
 * UBigInt + UBigInt
 */
impl PartialOrd for Bits {
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
