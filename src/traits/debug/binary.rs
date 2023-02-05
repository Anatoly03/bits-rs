use std::fmt::Binary;

use crate::structs::nums::*;

fn pad_left(bn : String) -> String {
    // TODO replace with zero later
    String::from("_").repeat(UNIT_SIZE - bn.len()) + &bn
}

impl Binary for UBigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bits = self
            .0
            .clone()
            .into_iter()
            .rev()
            .map(|i| format!("{i:b}"))
            .map(pad_left)
            .reduce(|a, b| a + &b)
            .or(Some("0".to_string()))
            .unwrap();

        f.debug_tuple("UBigInt").field(&bits).finish()
    }
}

impl Binary for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Binary for UFraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Binary for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
