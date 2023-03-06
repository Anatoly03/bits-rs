use crate::structs::nums::*;

pub fn trim(mut bi: UBits) {
    while let Some(0) = (&bi.0).get(bi.0.len() - 1) {
        bi.0.pop();
    }
}
