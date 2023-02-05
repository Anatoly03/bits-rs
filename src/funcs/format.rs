use crate::structs::nums::*;
use crate::structs::consts::UNIT_SIZE;

pub fn pad_left(bn: String, length: usize) -> String {
    String::from("0").repeat(length - bn.len()) + &bn
}

impl UBigInt {
    pub fn to_bin(&self) -> String {
        self.0
            .clone()
            .into_iter()
            .rev()
            .map(|i| format!("{i:b}"))
            .map(|i| pad_left(i, UNIT_SIZE))
            .reduce(|a, b| a + &b)
            .or(Some("0".to_string()))
            .unwrap()
    }

    pub fn to_octal(&self) -> String {
        self.0
            .clone()
            .into_iter()
            .rev()
            .map(|i| format!("{i:o}"))
            .map(|i| pad_left(i, UNIT_SIZE / 2))
            .reduce(|a, b| a + &b)
            .or(Some("0".to_string()))
            .unwrap()
    }

    pub fn to_hexadecimal(&self) -> String {
        self.0
            .clone()
            .into_iter()
            .rev()
            .map(|i| format!("{i:X}"))
            .map(|i| pad_left(i, UNIT_SIZE / 4))
            .reduce(|a, b| a + &b)
            .or(Some("0".to_string()))
            .unwrap()
    }
}
