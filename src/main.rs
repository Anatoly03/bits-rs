use crate::structs::nums::UBits;
use crate::structs::consts::Atomic;

pub mod structs;
pub mod impls;
pub mod funcs;
pub mod traits;
pub mod macros;
pub mod test;

fn main() {
    let mut big_int = UBits (vec![255, 10]);
    let big_int_2 = UBits (vec![Atomic::MAX, 10]);

    let test = num!("3124792073401784501945");
    
    println!("{:X}", big_int);
    big_int.set_bit(5);
    println!("{:b}", big_int);

    for bit_index in big_int.clone().bit_iter() {
        println!("{}", bit_index);
    }
}
