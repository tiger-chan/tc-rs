pub mod split_mix;
pub mod xorshiro;
mod prng_32;
mod prng_64;
mod prng_128;

pub trait Prng32<T> {
    fn next(&mut self) -> T;
}

pub trait Prng64<T> {
    fn next(&mut self) -> T;
}

pub trait Prng128<T> {
    fn next(&mut self) -> T;
}

pub use xorshiro::xorshiro;
pub use split_mix::split_mix;

pub mod prelude {
    pub use super::{
        xorshiro,
        split_mix,
        Prng64,
        xorshiro::Xorshiro64,
        split_mix::SplitMix64,
    };
}

