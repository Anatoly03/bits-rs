use std::fmt::Debug;

use crate::structs::nums::*;

impl Debug for UBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UBigInt").field(&self.0).finish()
    }
}
