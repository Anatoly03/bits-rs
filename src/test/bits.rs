#[cfg(test)]
mod format {
    use crate::structs::{nums::Bits, consts::Atomic};

    #[test]
    fn set_bit() {
        let mut big_int = Bits::from(vec![1, 10]);

        assert!(format!("{:b}", big_int).ends_with("00000001"));
        big_int.set_bit(5);
        assert!(format!("{:b}", big_int).ends_with("00100001"));
    }

    #[test]
    fn bit_iter() {
        let big_int = Bits::from(vec![0b00010000, 0b00010001]);
        let mut bit_iter = big_int.bit_iter();

        assert_eq!(Some(0), bit_iter.next());
        assert_eq!(Some(4), bit_iter.next());
        assert_eq!(Some((4 + Atomic::BITS) as usize), bit_iter.next());
        assert_eq!(None, bit_iter.next());
    }

    #[test]
    fn cardinality() {
        let big_int = Bits::from(vec![0b00010000, 0b00010001]);
        
        assert_eq!(3, big_int.cardinality());
    }

    #[test]
    fn length() {
        let big_int = Bits::from(vec![0b00010000, 0b00010001]);
        
        assert_eq!((4 + Atomic::BITS) as usize, big_int.len());
    }
}
