use std::ops::Shl;

use crate::structs::nums::*;

impl Shl for UBits {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Shl for Bits {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

// impl Shl for UFraction {
//     type Output = Self;

//     fn shl(self, rhs: Self) -> Self::Output {
//         todo!()
//     }
// }

// impl Shl for Fraction {
//     type Output = Self;

//     fn shl(self, rhs: Self) -> Self::Output {
//         todo!()
//     }
// }
