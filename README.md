# 2023-09-10-Elliptic-Curve-in-Rust
- Study elliptic curve by coding
- Practice operator overloading by creating my own struct over Z/pZ.
- **Main Takeaway.** 
  - Learned how to define the interface for the operator to overload. 
  - For example, I was trapped by the mindset of computing everything through reference (no copy, no move), but then I cannot return computational result since every value has a reference to a local variable inside a function! 
  - I need to design an interface to move data ownership "downwards" in order to return a valid data.
