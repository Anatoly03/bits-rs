use crate::structs::nums::UBigInt;

pub mod structs;
pub mod traits;

fn main() {
    let big_int = UBigInt (vec![7, 8, 1]);
 
    println!("{:b}", big_int);
}
