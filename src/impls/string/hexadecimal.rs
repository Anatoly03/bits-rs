use std::fmt::{UpperHex, LowerHex};

use crate::structs::nums::*;

impl UpperHex for UBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", &self.to_hexadecimal()))
    }
}

impl LowerHex for UBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", &self.to_hexadecimal().to_lowercase()))
    }
}
