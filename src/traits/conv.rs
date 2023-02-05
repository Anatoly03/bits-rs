
use crate::structs::nums::*;

/**
 * UBigInt -> BigInt
 */
impl From<UBigInt> for BigInt {
    fn from(this: UBigInt) -> Self {
        BigInt(false, this)
    }
}

/**
 * BigInt -> UBigInt
 */
impl From<BigInt> for UBigInt {
    fn from(this: BigInt) -> Self {
        if this.0 {
            panic!("Tried to parse negative BigInt to unsigned!")
        }
        this.1
    }
}