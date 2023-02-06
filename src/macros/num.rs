/**
 * Converts a number in decimal notation to big int
 */
#[macro_export]
macro_rules! num {
    () => {
        UBits(vec![])
    };

    ( $x:expr ) => {
        {
            println!("{}", $x);

            UBits(vec![])
        }
    };
}
