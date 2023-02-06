use crate::structs::nums::*;

/**
 * UBigInt -> BigInt
 */
impl Into<BigInt> for UBits {
    fn into(self) -> BigInt {
        BigInt(false, self)
    }
}

/**
 * BigInt -> UBigInt
 */
impl TryInto<UBits> for BigInt {
    type Error = String;

    fn try_into(self) -> Result<UBits, Self::Error> {
        if self.0 {
            Err(String::from("Tried to parse negative BigInt to unsigned!"))
        } else {
            Ok(self.1)
        }
    }
}
