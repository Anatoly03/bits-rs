use std::fmt::Octal;

use crate::structs::nums::*;

impl Octal for Bits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", &self.to_octal()))
    }
}
