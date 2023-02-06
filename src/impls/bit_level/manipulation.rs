use crate::structs::nums::UBits;
use crate::structs::consts::UNIT_SIZE;

impl UBits {
    /**
     * Sets bit at `pos` to `1`
     */
    pub fn set_bit(&mut self, pos: usize) {
        let index = pos / UNIT_SIZE;
        let count = if index > self.0.len() {
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

    /**
     * Get bit at `pos` as bool.
     * Returns `None` if position out of bounds
     * Returns `Some(bool)` 
     */
    pub fn get_bit(&mut self, pos: usize) -> Option<bool> {
        let index = pos / UNIT_SIZE;

        if index > self.0.len() {
            None
        } else {
            let bit_pos = pos % UNIT_SIZE;
            let bit = (*self.0.get(index).unwrap() >> bit_pos) % 2 == 1;
            
            Some(bit)
        }
    }
}