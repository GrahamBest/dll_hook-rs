# dll_hook-rs
Rust code to show how hooking in rust with a dll works.

# How to use
Replace the 
```rust
std::mem::transmute::<usize, createmove_fn>(0x10111790)
```
address on this line, with the address to your function.

Createmove is an example, no, its not intended to be used unless youre reversing source engine.
