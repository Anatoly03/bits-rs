use std::str::FromStr;

use crate::structs::nums::*;

impl FromStr for UBigInt {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl FromStr for BigInt {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}