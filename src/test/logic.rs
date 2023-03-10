
#[cfg(test)]
mod bit_or {
    use crate::structs::nums::Bits;

    #[test]
    fn bit_or_zero() {
        let i = Bits::new();
        let j = Bits::new();

        // Swapped because of type problems
        assert_eq!(i.0, vec![]);
        assert_eq!(j.0, vec![]);

        let k = &i | &j;

        assert_eq!(k.0, vec![]);
    }

    #[test]
    fn bit_or() {
        let i = Bits::from(vec![0b00110101]);
        let j = Bits::from(vec![0b01100011]);

        let k = &i | &j;

        assert_eq!(vec![0b01110111], k.0);
    }
}

#[cfg(test)]
mod bit_xor {
    use crate::{structs::nums::Bits};

    #[test]
    fn bit_xor_zero() {
        let i = Bits::new();
        let j = Bits::new();

        // Swapped because of type problems
        assert_eq!(i.0, vec![]);
        assert_eq!(j.0, vec![]);

        let k = &i ^ &j;

        assert_eq!(k.0, vec![]);
    }

    #[test]
    fn bit_xor() {
        let i = Bits::from(vec![0b00110101]);
        let j = Bits::from(vec![0b01100011]);

        let k = &i ^ &j;

        assert_eq!(vec![0b10101001], k.0);
    }
}

#[cfg(test)]
mod bit_and {
    use crate::{structs::nums::Bits};

    #[test]
    fn bit_and_zero() {
        let i = Bits::new();
        let j = Bits::new();

        // Swapped because of type problems
        assert_eq!(i.0, vec![]);
        assert_eq!(j.0, vec![]);

        let k = &i & &j;

        assert_eq!(k.0, vec![]);
    }

    #[test]
    fn bit_and() {
        let i = Bits::from(vec![0b00110101]);
        let j = Bits::from(vec![0b01100011]);

        let k = &i & &j;

        assert_eq!(vec![0b00100001], k.0);
    }
}