use std::fmt::Binary;

use crate::structs::nums::*;

impl Binary for UBigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", &self.to_bin()))
    }
}

impl Binary for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", if self.0 { "-" } else { "" }, &self.1.to_bin()))
    }
}