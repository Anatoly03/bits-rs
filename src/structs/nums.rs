use crate::Atomic;

/**
 * Type `UBigInt`
 * Param `0`: Vector of all
 */
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct UBits(pub Vec<Atomic>);

/**
 * Type `BigInt`
 * Param `0` : Sign (True -> Negative)
 * Param `1` : Data
 */
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Bits(pub bool, pub UBits);

/**
 * Type `BitIterator`
 * Absorbs a a UBits and creates an iterator over the bits.
 */
#[derive(Hash)]
pub struct BitIter {
    pub source: UBits,
    pub current: usize
}
