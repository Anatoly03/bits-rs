use crate::structs::nums::UBigInt;

pub mod structs;
pub mod traits;

fn main() {
    let big_int = UBigInt (vec![1, 99000000]);
    let big_int_2 = UBigInt (vec![u64::MAX, 1]);
    let sum = &big_int + &big_int_2;
 
    println!("{:b}", big_int);
    println!("{:b}", big_int_2);
    println!("{:b}", sum);
}
