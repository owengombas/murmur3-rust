# 🌀 Murmurs: A Reliable Murmur3 Hash in Rust

This crate provides a **reliable and production-ready implementation** of the [Murmur3 hash function](https://github.com/aappleby/smhasher/blob/master/src/MurmurHash3.cpp), based directly on the reference C++ implementation.

The core logic is in [`src/murmur3.rs`](src/murmur3.rs), and it's tested against the original via C bindings.

## 📦 Installation
Add the following to your `Cargo.toml`:

```toml
[dependencies]
murmurs = "1.0.0" # or the latest published version
```

## 🚀 Usage
```rust
use murmurs::murmur3_x86_32;

fn main() {
    let hash = murmur3_x86_32(b"hello world", 0);
    println!("{}", hash);
}
```

Then run:
```sh
cargo run
```

## ✅ Testing
The implementation is automatically tested against the reference [C++ implementation from smhasher](https://github.com/aappleby/smhasher/blob/master/src/MurmurHash3.cpp) using FFI bindings.

To run the tests:
```sh
cargo test
```

## 🔒 Status
- ✅ Verified against the original MurmurHash3.cpp
- ✅ Consistent 32-bit and 128-bit output
- ✅ Portable and deterministic
 