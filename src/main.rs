use crate::impls::new::New;
use crate::structs::nums::UBits;
use crate::structs::consts::Atomic;

pub mod structs;
pub mod impls;
pub mod funcs;
pub mod traits;
pub mod macros;
pub mod test;

fn main() {
    let mut big_int = UBits (vec![1, 10]);
    let big_int_2 = UBits (vec![Atomic::MAX, 10]);
    let big_zero = UBits::new();
    let sum = &big_int + &big_int_2;

    let test = num!("3124792073401784501945");
    
    println!("{:b}", big_int);
    big_int.set_bit(5);
    println!("{:b}", big_int);

    for bit_index in big_int.clone().bit_iter() {
        println!("{}", bit_index);
    }
    
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
