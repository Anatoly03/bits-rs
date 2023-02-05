
use crate::structs::nums::*;

impl Clone for UBigInt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

// impl Clone for UFraction {
//     fn clone(&self) -> Self {
//         Self(self.0.clone(), self.1.clone())
//     }
// }

// impl Clone for Fraction {
//     fn clone(&self) -> Self {
//         Self(self.0.clone(), self.1.clone())
//     }
// }