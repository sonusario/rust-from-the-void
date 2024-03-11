# Scratch Paper for Rust

A place to keep track of Rust code that I play around with.

To run a module, at the top of `main.rs` write:

```rust
 mod module_name;
 use module_name::*;
```

In `fn main() { ... }` write: `<module-name>();`

Module names can be found in the `./src/` folder, each being called
`<module-name>.rs`.

The module currently in `fn main() { ... }` is the one most recently made/used.