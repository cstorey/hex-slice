# hex-slice

[![Build Status](https://travis-ci.org/cstorey/hex-slice.svg?branch=master)](https://travis-ci.org/cstorey/hex-slice)

Renders a slice of integers (or anythign lese that supports the
[LowerHex](https://doc.rust-lang.org/std/fmt/trait.LowerHex.html) or [UpperHex](https://doc.rust-lang.org/std/fmt/trait.UpperHex.html) traits as hex. For example, this:

```rust
extern crate hex_slice;
use hex_slice::AsHex;

fn main() {
    let foo = vec![0u32, 1 ,2 ,3];
    println!("{:x}", foo.as_hex());
}
```

Displays: `[0000 0001 0002 0003]` on standard output.

