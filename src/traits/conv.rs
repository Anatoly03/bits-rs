
use std::fmt::Error;

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
impl TryInto<UBigInt> for BigInt {
    type Error = String;

    fn try_into(self) -> Result<UBigInt, Self::Error> {
        if self.0 {
            Err(String::from("Tried to parse negative BigInt to unsigned!"))
        } else {
            Ok(self.1)
        }
    }
}