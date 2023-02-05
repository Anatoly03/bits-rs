use crate::structs::nums::UBigInt;
use crate::structs::consts::Atomic;

pub mod structs;
pub mod impls;
pub mod funcs;
pub mod traits;

fn main() {
    let big_int = UBigInt (vec![1, 10]);
    let big_int_2 = UBigInt (vec![Atomic::MAX, 10]);
    let big_zero = UBigInt (vec![255]);
    let sum = &big_int + &big_int_2;
    
    println!("{:b}", big_int);
    println!("{:X}", big_int);
    // println!("{}", big_int.to_decimal_string());
    println!("{:b}", big_int_2);
    println!("{:X}", big_int_2);
    // println!("{}", big_int_2.to_decimal_string());
    println!("{:b}", sum);
    println!("{:X}", sum);
    println!("{:X}", big_zero);
    // println!("{}", sum.to_decimal_string());
    // println!("{}", big_small.to_decimal_string());
}
