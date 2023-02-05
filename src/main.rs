use crate::structs::nums::{UBigInt, Atomic};

pub mod structs;
pub mod impls;
pub mod funcs;
pub mod traits;

fn main() {
    let big_int = UBigInt (vec![1, 10]);
    let big_int_2 = UBigInt (vec![Atomic::MAX, 10]);
    let sum = &big_int + &big_int_2;
 
    println!("{:b}", big_int);
    println!("{:b}", big_int_2);
    println!("{:b}", sum);

    64i32.signum();
}
