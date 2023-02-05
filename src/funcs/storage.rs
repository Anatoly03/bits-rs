use crate::structs::nums::*;

pub fn trim(mut bi: UBigInt) {
    while let Some(0) = (&bi.0).get(bi.0.len() - 1) {
        bi.0.pop();
    }
}

pub fn set_sign_for_0(mut bi: BigInt) {
    if bi.1.0.len() == 0 {
        bi.0 = false;
    }
}