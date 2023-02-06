use crate::structs::nums::BitIter;
use crate::structs::consts::UNIT_SIZE;

impl Iterator for BitIter {
    type Item = usize;

    /**
     * 00110011 11001100
     * 0      7 8      15
     */
    fn next(&mut self) -> Option<Self::Item> {
        let mut index = self.current / UNIT_SIZE;
        
        while let Some(byte) = self.source.0.get(index) {
            let bit = self.current % UNIT_SIZE;
            let value = (byte >> bit) % 2; //& 0x01;
            
            self.current += 1;

            if value == 1 {
                return Some(self.current - 1);
            } else {
                index = self.current / UNIT_SIZE;
            }
        }

        None
    }
}
