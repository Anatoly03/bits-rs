use crate::structs::nums::*;

impl Clone for UBits {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Clone for Bits {
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
