use crate::{
    prng_128::Prng128, prng_32::Prng32, prng_64::Prng64, split_mix::*, Prng128 as _, Prng32 as _,
    Prng64 as _,
};

pub trait XorshiroNew<T> {
    fn make(self) -> Xorshiro<T>;
}

impl XorshiroNew<u32> for u32 {
    fn make(self) -> Xorshiro<u32> {
        Xorshiro32::new(self)
    }
}

impl XorshiroNew<u64> for u64 {
    fn make(self) -> Xorshiro<u64> {
        Xorshiro64::new(self)
    }
}

impl XorshiroNew<u128> for u128 {
    fn make(self) -> Xorshiro<u128> {
        Xorshiro128::new(self)
    }
}

pub fn xorshiro<T>(seed: T) -> Xorshiro<T>
where T : XorshiroNew<T> {
    seed.make()
}

pub struct Xorshiro<T> {
    seed: T,
}

impl Xorshiro<u32> {
    pub fn new(seed: u32) -> Self {
        Self {
            seed: SplitMix::new(seed).next(),
        }
    }

    fn xorshiro(mut x: u32) -> u32 {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        x = x.rotate_left(22);
        x
    }
}

impl Xorshiro<u64> {
    pub fn new(seed: u64) -> Self {
        Self {
            seed: SplitMix::new(seed).next(),
        }
    }

    fn xorshiro(mut x: u64) -> u64 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x = x.rotate_left(45);
        x
    }
}

impl Xorshiro<u128> {
    pub fn new(seed: u128) -> Self {
        Self {
            seed: SplitMix::new(seed).next(),
        }
    }

    fn xorshiro(mut x: u128) -> u128 {
        x ^= x << 17;
        x ^= x >> 13;
        x ^= x << 7;
        x = x.rotate_left(64);
        x
    }
}

impl Default for Xorshiro<u32> {
    fn default() -> Self {
        Self {
            seed: SplitMix::new(0u32).next(),
        }
    }
}

impl Default for Xorshiro<u64> {
    fn default() -> Self {
        Self {
            seed: SplitMix::new(0u64).next(),
        }
    }
}

impl Default for Xorshiro<u128> {
    fn default() -> Self {
        Self {
            seed: SplitMix::new(0u128).next(),
        }
    }
}

impl Prng32 for Xorshiro<u32> {
    fn calc(&mut self) -> u32 {
        let x = self.seed.wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        self.seed = Self::xorshiro(self.seed);
        x
    }
}

impl Prng64 for Xorshiro<u64> {
    fn calc(&mut self) -> u64 {
        let x = self.seed.wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        self.seed = Self::xorshiro(self.seed);
        x
    }
}

impl Prng128 for Xorshiro<u128> {
    fn calc(&mut self) -> u128 {
        let x = self.seed.wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        self.seed = Self::xorshiro(self.seed);
        x
    }
}

pub type Xorshiro32 = Xorshiro<u32>;
pub type Xorshiro64 = Xorshiro<u64>;
pub type Xorshiro128 = Xorshiro<u128>;
