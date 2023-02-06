use crate::impls::new::New;
use crate::structs::nums::UBigInt;
use crate::structs::consts::Atomic;

pub mod structs;
pub mod impls;
pub mod funcs;
pub mod traits;
pub mod macros;

fn main() {
    let big_int = UBigInt (vec![1, 10]);
    let big_int_2 = UBigInt (vec![Atomic::MAX, 10]);
    let big_zero = UBigInt::new();
    let sum = &big_int + &big_int_2;

    let test = num!("3124792073401784501945");
    
    // println!("{:b}", big_int);
    // println!("{:b}", big_int_2);
    // println!("{:b}", sum);

    println!("{:X} + {:X} = {:X}", big_int, big_int_2, sum);
    // println!("{}", big_int.to_decimal_string());
    // println!("{:X}", big_int_2);
    // println!("{}", big_int_2.to_decimal_string());
    // println!("{:X}", sum);
    println!("{:X}", big_zero);
    // println!("{}", sum.to_decimal_string());
    // println!("{}", big_small.to_decimal_string());
}
