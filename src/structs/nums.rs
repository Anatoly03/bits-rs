use crate::Atomic;

/**
 * Type `UBigInt`
 * Param `0`: Vector of all
 */
#[derive(PartialEq, Eq, Hash)]
pub struct UBits(pub Vec<Atomic>);

/**
 * Type `BigInt`
 * Param `0` : Sign (True -> Negative)
 * Param `1` : Data
 */
#[derive(PartialEq, Eq, Hash)]
pub struct Bits(pub bool, pub UBits);

