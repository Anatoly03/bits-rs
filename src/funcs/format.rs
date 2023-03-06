use crate::structs::consts::UNIT_SIZE;
use crate::structs::nums::*;

fn pad_left(bn: String, length: usize) -> String {
    String::from("0").repeat(length - bn.len()) + &bn
}

fn unpad_left(mut num: String) -> String {
    while let Some(i) = num.get(0..1) {
        if i == "0" {
            num.remove(0);
            continue;
        }
        break;
    }
    num
}

impl Bits {
    pub fn to_bin(&self) -> String {
        let s = self
            .0
            .clone()
            .into_iter()
            .rev()
            .map(|i| format!("{i:b}"))
            .map(|i| pad_left(i, UNIT_SIZE))
            .reduce(|a, b| a + &b)
            .map(unpad_left)
            .or(Some("0".to_string()))
            .unwrap();

        if s.is_empty() {
            return String::from("0");
        }
        s
    }

    pub fn to_octal(&self) -> String {
        let s = self
            .0
            .clone()
            .into_iter()
            .rev()
            .map(|i| format!("{i:o}"))
            .map(|i| pad_left(i, UNIT_SIZE / 2))
            .reduce(|a, b| a + &b)
            .map(unpad_left)
            .or(Some("0".to_string()))
            .unwrap();

        if s.is_empty() {
            return String::from("0");
        }
        s
    }

    pub fn to_hexadecimal(&self) -> String {
        let s = self
            .0
            .clone()
            .into_iter()
            .rev()
            .map(|i| format!("{i:X}"))
            .map(|i| pad_left(i, UNIT_SIZE / 4))
            .reduce(|a, b| a + &b)
            .map(unpad_left)
            .or(Some("0".to_string()))
            .unwrap();

        if s.is_empty() {
            return String::from("0");
        }
        s
    }
}
