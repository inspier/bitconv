[![Current Crates.io Version](https://img.shields.io/crates/v/bitconv.svg)](https://crates.io/crates/bitconv)
[![docs-rs](https://docs.rs/bitconv/badge.svg)](https://docs.rs/bitconv)

# bitconv

A simple, zero-dependency, no_std compatible Rust equivalent to the C# BitConverter class.

To add to your Cargo.toml:
```toml
bitconv = "0.1.2"
```

## Example
```rust
use bitconv::{endian::{Big, Little, Native}, to_uint32};

fn main() {
    let buffer = [15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 19];
    assert_eq!(261888, to_uint32::<Little>(&buffer, 6));
    assert_eq!(261888, to_uint32::<Native>(&buffer, 6));
    assert_eq!(16712448, to_uint32::<Big>(&buffer, 6));
}
```
