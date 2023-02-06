use std::cmp::min;

use crate::structs::nums::{UBits, BitIter};
use crate::structs::consts::UNIT_SIZE;

impl UBits {
    pub fn set_bit(&mut self, pos: usize) {
        let index = pos / UNIT_SIZE;
        let count = if (index > self.0.len()) {
            index - self.0.len()
        } else {
            0
        };

        for _ in 0..count {
            self.0.push(0);
        }

        let mut byte = *self.0.get(index).unwrap();
        let bit = pos % UNIT_SIZE;
        byte ^= 1 << bit;
        self.0[index] = byte;
    }
}