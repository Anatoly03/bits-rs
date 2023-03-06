use std::fmt::Binary;

use crate::structs::nums::*;

impl Binary for UBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", &self.to_bin()))
    }
}
