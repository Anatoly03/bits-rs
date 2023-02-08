# Ideas

```rs
struct Bits {
    length, usize, // Allow end with 0 bits
    generator : [Fn(isize) -> Atomic], // Generator Method to fill in bits 
    data: Vec<Atomic> // Stored data
}

struct Number {
    source: [Bits, 2], // Pre Comma and Post Comma
    sign: Sign, // Negative, positive, zero, complex?
}
```

```rs
num!("13376942077123456789") // Converts decimal to binary number representation
```