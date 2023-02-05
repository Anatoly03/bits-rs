use crate::structs::nums::*;

pub fn pad_left(bn: String) -> String {
    // if bn.len() == UNIT_SIZE {
    //     return bn;
    // }
    String::from("_").repeat(UNIT_SIZE - bn.len()) + &bn
}

impl UBigInt {
    pub fn to_bin(&self) -> String {
        self.0
            .clone()
            .into_iter()
            .rev()
            .map(|i| format!("{i:b}"))
            .map(pad_left)
            .reduce(|a, b| a + &b)
            .or(Some("0".to_string()))
            .unwrap()
    }
}
