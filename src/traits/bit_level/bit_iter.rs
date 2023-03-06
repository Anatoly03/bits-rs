use crate::structs::nums::{Bits, BitIter};

impl From<Bits> for BitIter {
    fn from(bits: Bits) -> Self {
        BitIter {
            source: bits,
            current: 0,
        }
    }
}

impl Bits {
    pub fn bit_iter(self) -> BitIter {
        BitIter::from(self)
    }
}