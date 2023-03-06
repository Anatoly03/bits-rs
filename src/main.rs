use crate::structs::nums::Bits;
use crate::structs::consts::Atomic;

pub mod structs;
pub mod impls;
pub mod funcs;
pub mod macros;
pub mod test;

fn main() {
    let mut big_int = Bits (vec![1, 10]);
    let big_int_2 = Bits (vec![Atomic::MAX, 10]);

    let test = num!("3124792073401784501945");

    for bit_index in big_int.clone().bit_iter() {
        println!("{}", bit_index);
    }
}
