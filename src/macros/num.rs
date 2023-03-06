/**
 * Converts a number in decimal notation to big int
 */
#[macro_export]
macro_rules! num {
    () => {
        Bits(vec![])
    };

    ( $x:expr ) => {
        {
            println!("{}", $x);

            Bits(vec![])
        }
    };
}
