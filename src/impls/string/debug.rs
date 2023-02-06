use std::fmt::Debug;

use crate::structs::nums::*;

impl Debug for UBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UBigInt").field(&self.0).finish()
    }
}

impl Debug for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BigInt")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

// impl Debug for UFraction {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_tuple("UFraction").field(&self.0).field(&self.1).finish()
//     }
// }

// impl Debug for Fraction {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_tuple("Fraction").field(&self.0).field(&self.1).finish()
//     }
// }
