# 🌀 Murmurs: A Reliable Murmur3 Hash in Rust
This crate provides a **reliable and production-ready implementation** of the [Murmur3 hash function](https://github.com/aappleby/smhasher/blob/master/src/MurmurHash3.cpp), faithfully translated from the original C++ reference and usable in the same way.

The core logic resides in [`src/murmur3.rs`](src/murmur3.rs), and its correctness is verified against the latest version of the [official smhasher implementation](https://github.com/aappleby/smhasher/blob/master/src/MurmurHash3.cpp) through tests using C FFI bindings.

## 📦 Installation
Add the following to your `Cargo.toml`:

```toml
[dependencies]
murmurs = "1.0" # or the latest published version
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
- ✅ Verified against the original `MurmurHash3.cpp`
- ✅ Consistent 32-bit and 128-bit output
- ✅ Portable and deterministic
- ✅ Can be used the same way as the C++ reference
 