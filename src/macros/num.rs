/**
 * Converts a number in decimal notation to big int
 */
#[macro_export]
macro_rules! num {
    () => {
        UBigInt(vec![])
    };

    ( $x:expr ) => {
        {
            println!("{}", $x);

            UBigInt(vec![])
        }
    };
}
