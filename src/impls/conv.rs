use crate::structs::nums::*;

/**
 * UBigInt -> BigInt
 */
impl Into<Bits> for UBits {
    fn into(self) -> Bits {
        Bits(false, self)
    }
}

/**
 * BigInt -> UBigInt
 */
impl TryInto<UBits> for Bits {
    type Error = String;

    fn try_into(self) -> Result<UBits, Self::Error> {
        if self.0 {
            Err(String::from("Tried to parse negative BigInt to unsigned!"))
        } else {
            Ok(self.1)
        }
    }
}
