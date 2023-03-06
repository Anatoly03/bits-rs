
#[cfg(test)]
mod add {
    use crate::{structs::{nums::Bits, consts::Atomic}};

    #[test]
    fn add_zero() {
        let i = Bits::new();
        let j = Bits::new();

        // Swapped because of type problems
        assert_eq!(i.0, vec![]);
        assert_eq!(j.0, vec![]);

        let k = &i + &j;

        assert_eq!(k.0, vec![]);
    }

    #[test]
    fn add() {
        let i = Bits::from(vec![20]);
        let j = Bits::from(vec![40]);

        assert_eq!(vec![20], i.0);
        assert_eq!(vec![40], j.0);

        let k = &i + &j;

        assert_eq!(vec![60], k.0);
    }

    #[test]
    fn add_overflow() {
        let i = Bits::from(vec![Atomic::MAX - 1]);
        let j = Bits::from(vec![5]);

        let k = &i + &j;

        assert_eq!(vec![3, 1], k.0);
    }
}

mod sub {
    #[test]
    fn sub() {
        todo!("Not implemented yet")
    }
}

mod div {
    #[test]
    fn div() {
        todo!("Not implemented yet")
    }
}

mod rem {
    #[test]
    fn rem() {
        todo!("Not implemented yet")
    }
}

mod mul {
    #[test]
    fn mul() {
        todo!("Not implemented yet")
    }
}

mod shl {
    #[test]
    fn shl() {
        todo!("Not implemented yet")
    }
}

mod shr {
    #[test]
    fn shr() {
        todo!("Not implemented yet")
    }
}
