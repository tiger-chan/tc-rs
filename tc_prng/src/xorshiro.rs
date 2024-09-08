use crate::{prng_128 as p128, prng_32 as p32, prng_64 as p64, split_mix::*};

pub trait XorshiroNew<T> {
    fn make(seed: T) -> Xorshiro<T>;
}

impl XorshiroNew<u32> for Xorshiro<u32> {
    fn make(seed: u32) -> Xorshiro<u32> {
        Xorshiro32 {
            seed: SplitMix::new(seed).next(),
        }
    }
}

impl XorshiroNew<u64> for Xorshiro<u64> {
    fn make(seed: u64) -> Xorshiro<u64> {
        Xorshiro64 {
            seed: SplitMix::new(seed).next(),
        }
    }
}

impl XorshiroNew<u128> for Xorshiro<u128> {
    fn make(seed: u128) -> Xorshiro<u128> {
        Xorshiro128 {
            seed: SplitMix::new(seed).next(),
        }
    }
}

pub fn xorshiro<T>(seed: T) -> Xorshiro<T>
where
    Xorshiro<T>: XorshiroNew<T>,
{
    Xorshiro::<T>::new(seed)
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Xorshiro<T> {
    seed: T,
}

impl<T> Xorshiro<T>
where
    Xorshiro<T>: XorshiroNew<T>,
{
    fn new(seed: T) -> Self {
        Xorshiro::make(seed)
    }
}

impl Xorshiro<u32> {
    fn xorshiro(mut x: u32) -> u32 {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        x = x.rotate_left(22);
        x
    }

    pub fn next<V>(&mut self) -> V
    where
        Self: p32::Prng<V>,
    {
        p32::Prng::next_val(self)
    }
}

impl Xorshiro<u64> {
    fn xorshiro(mut x: u64) -> u64 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x = x.rotate_left(45);
        x
    }

    pub fn next<V>(&mut self) -> V
    where
        Self: p64::Prng<V>,
    {
        p64::Prng::next_val(self)
    }
}

impl Xorshiro<u128> {
    fn xorshiro(mut x: u128) -> u128 {
        x ^= x << 17;
        x ^= x >> 13;
        x ^= x << 7;
        x = x.rotate_left(64);
        x
    }

    pub fn next<V>(&mut self) -> V
    where
        Self: p128::Prng<V>,
    {
        p128::Prng::next_val(self)
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

impl p32::Prng32 for Xorshiro<u32> {
    fn calc(&mut self) -> u32 {
        let x = self.seed.wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        self.seed = Self::xorshiro(self.seed);
        x
    }
}

impl p64::Prng64 for Xorshiro<u64> {
    fn calc(&mut self) -> u64 {
        let x = self.seed.wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        self.seed = Self::xorshiro(self.seed);
        x
    }
}

impl p128::Prng128 for Xorshiro<u128> {
    fn calc(&mut self) -> u128 {
        let x = self.seed.wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        self.seed = Self::xorshiro(self.seed);
        x
    }
}

pub type Xorshiro32 = Xorshiro<u32>;
pub type Xorshiro64 = Xorshiro<u64>;
pub type Xorshiro128 = Xorshiro<u128>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dump_value_types() {
        let mut rng = xorshiro(123456_u64);

        let a: u8 = rng.next();
        assert_eq!(a, 10);

        let b: u128 = rng.next();
        assert_eq!(b, 183097476445210036426531508735958283909);

        let c: bool = rng.next();
        assert_eq!(c, true);

        let d: f32 = rng.next();
        assert_eq!(d, 0.074927814);
    }
}
