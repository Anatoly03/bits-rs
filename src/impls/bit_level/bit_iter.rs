use crate::structs::nums::{UBits, BitIter};

impl From<UBits> for BitIter {
    fn from(bits: UBits) -> Self {
        BitIter {
            source: bits,
            current: 0,
        }
    }
}

impl UBits {
    pub fn bit_iter(self) -> BitIter {
        BitIter::from(self)
    }
}