#[cfg(test)]
mod format {
    use crate::structs::nums::Bits;

    #[test]
    fn binary() {
        let big_int = Bits::from(vec![1, 10]);
        let formatted = format!("{:b}", big_int);

        assert!(formatted.starts_with("1010")); // Started with "10"
        // Contains zeroes depending on the size of ATOMIC
        assert!(formatted.ends_with("00000001")); // Ends with a byte that equals 1
    }

    #[test]
    fn octal() {
        let big_int = Bits::from(vec![255, 10]);
        let formatted = format!("{:o}", big_int);

        assert!(formatted.starts_with("12")); // Started with "10"
        // Contains zeroes depending on the size of ATOMIC
        assert!(formatted.ends_with("377")); // Ends with a byte that equals 1
    }

    #[test]
    fn hex() {
        let big_int = Bits::from(vec![255, 10]);
        let formatted = format!("{:X}", big_int);

        assert!(formatted.starts_with("A")); // Started with "10"
        // Contains zeroes depending on the size of ATOMIC
        assert!(formatted.ends_with("FF")); // Ends with a byte that equals 1
    }
}