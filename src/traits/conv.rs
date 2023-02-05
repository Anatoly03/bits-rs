
use crate::structs::nums::*;

/**
 * UBigInt -> BigInt
 */
impl Into<BigInt> for UBigInt {
    fn into(self) -> BigInt {
        BigInt(false, self)
    }
}

/**
 * BigInt -> UBigInt
 */
impl Into<UBigInt> for BigInt {
    fn into(self) -> UBigInt {
        if self.0 {
            panic!("Tried to parse negative BigInt to unsigned!")
        }
        self.1
    }
}