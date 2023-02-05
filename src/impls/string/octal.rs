use std::fmt::Octal;

use crate::structs::nums::*;

impl Octal for UBigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", &self.to_octal()))
    }
}

impl Octal for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", if self.0 { "-" } else { "" }, &self.1.to_octal()))
    }
}