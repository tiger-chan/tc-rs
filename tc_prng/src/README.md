# tc_prng

## Overview

`tc_prng` is a Rust library that provides multiple pseudo-random number
generators (PRNGs). They can be used in 32-bit, 64-bit, and 128-bit modes.
These PRNGs offer efficient, low-memory random number generation for various
use cases, and the interface is designed to be simple and easy to use.

## Features
- Supports 32-bit, 64-bit, and 128-bit modes
- Generic interface for various number types (u8, i64, f32, etc.)
- Seedable using an initial state (e.g., a u64 seed)

Installation
To add `tc_prng` to your Rust project, include the following in your
Cargo.toml:

```toml
[dependencies]
tc_prng = { git = "https://github.com/tiger-chan/tc-rs" }
```

## Example Usage

Here’s a basic example of how to use the TripleBuffer in a producer-consumer
scenario:

```rust
use tc_prng::prelude::*; 

fn main() {
    // Using SplitMix PRNG
    let mut rng = split_mix(12345_u64);
    let a: u8 = rng.next();
    let b: i64 = rng.next();
    let c: f32 = rng.next();

    println!("{}, {}, {}", a, b, c);

    // Using XorShiro PRNG
    let mut rng = xorshiro(12345_u64);
    let a: u8 = rng.next();
    let b: i64 = rng.next();
    let c: f32 = rng.next(); 

    println!("{}, {}, {}", a, b, c);
}
```

PRNGs share the same interface, making it easy to switch between the two
depending on your use case. The next() method is generic and will produce a
value of the requested type, be it u8, i64, f32, or any other supported type.

## Available PRNGs

### SplitMix
A fast, non-cryptographic PRNG suitable for a wide range of applications. It
generates random values by repeatedly applying a mix function to an internal
state.

### XorShiro
XorShiro is a family of fast, high-quality PRNGs. It is based on the
XOR/shift/rotate operations and is well-suited for simulations, games, and
other applications requiring high performance.

## Testing

Testing was preformed via [ent](https://www.fourmilab.ch/random/)

### SplitMix
```
Entropy = 7.997434 bits per byte.

Optimum compression would reduce the size
of this 80000 byte file by 0 percent.

Chi square distribution for 80000 samples is 284.83, and randomly
would exceed this value 9.65 percent of the times.

Arithmetic mean value of data bytes is 127.0524 (127.5 = random).
Monte Carlo value for Pi is 3.152178804 (error 0.34 percent).
Serial correlation coefficient is 0.003675 (totally uncorrelated = 0.0).

```

### Xorshiro

```
Entropy = 7.997715 bits per byte.

Optimum compression would reduce the size
of this 80000 byte file by 0 percent.

Chi square distribution for 80000 samples is 253.19, and randomly
would exceed this value 52.02 percent of the times.

Arithmetic mean value of data bytes is 127.4308 (127.5 = random).
Monte Carlo value for Pi is 3.136578414 (error 0.16 percent).
Serial correlation coefficient is 0.001287 (totally uncorrelated = 0.0).

```
