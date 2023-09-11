# 2023-09-10-Elliptic-Curve-in-Rust

- Study elliptic curve via coding by following the [tutorial](https://www.udemy.com/course/elliptic-curve-cryptography-in-rust/)
- **My difference from the tutorial.**
  - In tutorial, all computations are over `BigUint`, all points are `Point::Coor(BigUint, BigUint)`.
  - In my code, I simplify computations by designing my own struct, my points are `Point::Coor(Field, Field)`.
- Practice operator overloading by creating my own struct over $\mathbb Z/p\mathbb Z$ (which I denote `Field` in the `lib.rs`).
- **Main Takeaway.**
  - Learned how to define the interface for the operator to overload.
  - For example, I was trapped by the mindset of computing everything through reference (no copy, no move), but then I cannot return computational result since every value has a reference to a local variable inside a function!
  - I need to design an interface to move data ownership "downwards" in order to return a valid data.
