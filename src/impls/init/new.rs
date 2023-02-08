use crate::structs::nums::*;

impl UBits {
    pub fn new() -> Self {
        UBits(Vec::new())
    }
}

impl Bits {
    pub fn new() -> Self {
        Bits(false, UBits::new())
    }
}