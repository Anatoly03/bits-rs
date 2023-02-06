use std::ops::Neg;

use crate::structs::nums::*;

impl Neg for UBits {
    type Output = Bits;

    fn neg(self) -> Self::Output {
        Bits(true, self)
    }
}

impl Neg for Bits {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Bits(!self.0, self.1)
    }
}

// impl Neg for UFraction {
//     type Output = Fraction;

//     fn neg(self) -> Self::Output {
//         Fraction(true, self)
//     }
// }

// impl Neg for Fraction {
//     type Output = Self;

//     fn neg(self) -> Self::Output {
//         Fraction(!self.0, self.1)
//     }
// }
