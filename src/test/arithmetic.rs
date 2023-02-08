
#[cfg(test)]
mod add {
    use crate::{structs::nums::UBits, impls::new::New};

    #[test]
    fn add_zero() {
        let i = UBits::new();
        let j = UBits::new();

        let k = &i + &j;

        assert_eq!(i.0, vec![]);
        assert_eq!(j.0, vec![]);
        assert_eq!(k.0, vec![]);
    }
}