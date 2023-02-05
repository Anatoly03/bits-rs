use std::fmt::Binary;

use crate::structs::nums::*;

impl Binary for UBigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UBigInt").field(&self.to_bin()).finish()
    }
}

impl Binary for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BigInt")
            .field(&(String::from(if self.0 { "-" } else { "" }) + &self.1.to_bin()))
            .finish()
    }
}

// impl Binary for UFraction {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

// impl Binary for Fraction {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
