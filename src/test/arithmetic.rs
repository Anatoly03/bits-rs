
#[cfg(test)]
mod add {
    use crate::{structs::{nums::UBits, consts::Atomic}};

    #[test]
    fn add_zero() {
        let i = UBits::new();
        let j = UBits::new();

        // Swapped because of type problems
        assert_eq!(i.0, vec![]);
        assert_eq!(j.0, vec![]);

        let k = &i + &j;

        assert_eq!(k.0, vec![]);
    }

    #[test]
    fn add() {
        let i = UBits::from(vec![20]);
        let j = UBits::from(vec![40]);

        assert_eq!(vec![20], i.0);
        assert_eq!(vec![40], j.0);

        let k = &i + &j;

        assert_eq!(vec![60], k.0);
    }

    #[test]
    fn add_overflow() {
        let i = UBits::from(vec![Atomic::MAX - 1]);
        let j = UBits::from(vec![5]);

        let k = &i + &j;

        assert_eq!(vec![3, 1], k.0);
    }
}
