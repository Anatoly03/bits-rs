#[cfg(test)]
mod format {
    use crate::structs::nums::UBits;

    #[test]
    fn set_bit() {
        let mut big_int = UBits::from(vec![1, 10]);

        assert!(format!("{:b}", big_int).ends_with("00000001"));
        big_int.set_bit(5);
        assert!(format!("{:b}", big_int).ends_with("00100001"));
    }
}