
/**
 * The atomic unit of storage
 */
pub type Atomic = u8;
pub const UNIT_SIZE : usize = 8;

/**
 * Type `UBigInt`
 * Param `0`: Vector of all 
 */
#[derive(PartialEq, Eq, Hash)]
pub struct UBigInt (pub Vec<Atomic>);

/**
 * Type `BigInt`
 * Param `0` : Sign (True -> Negative)
 * Param `1` : Data
 */
#[derive(PartialEq, Eq, Hash)]
pub struct BigInt (pub bool, pub UBigInt);

/*
 * Type `UFraction`
 * Param `0` : numerator
 * Param `1` : denominator
 */
// pub struct UFraction (pub UBigInt, pub UBigInt);

/*
 * Type `Fraction`
 * Param `0` : sign
 * Param `1` : fraction
 */
// pub struct Fraction (pub bool, pub UFraction);

