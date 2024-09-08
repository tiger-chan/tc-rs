use crate::{prng_128 as p128, prng_32 as p32, prng_64 as p64};

pub fn split_mix<T>(seed: T) -> SplitMix<T> {
    SplitMix::new(seed)
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct SplitMix<T> {
    state: T,
}

impl Default for SplitMix<u32> {
    fn default() -> Self {
        Self { state: 0 }
    }
}

impl Default for SplitMix<u64> {
    fn default() -> Self {
        Self { state: 0 }
    }
}

impl Default for SplitMix<u128> {
    fn default() -> Self {
        Self { state: 0 }
    }
}

impl<T> SplitMix<T> {
    pub fn new(seed: T) -> Self {
        Self { state: seed }
    }
}

impl SplitMix<u32> {
    const ADD: u32 = 0x9e3779b9;
    const MU1: u32 = 0xbf58476d;
    const MU2: u32 = 0x94d049bb;
    const SH1: usize = 15;
    const SH2: usize = 13;
    const SH3: usize = 16;

    pub fn next<V>(&mut self) -> V
    where
        Self: p32::Prng<V>,
    {
        p32::Prng::next_val(self)
    }
}

impl SplitMix<u64> {
    const ADD: u64 = 0x9e3779b9_7f4a7c15;
    const MU1: u64 = 0xbf58476d_1ce4e5b9;
    const MU2: u64 = 0x94d049bb_133111eb;
    const SH1: usize = 30;
    const SH2: usize = 27;
    const SH3: usize = 31;

    pub fn next<V>(&mut self) -> V
    where
        Self: p64::Prng<V>,
    {
        p64::Prng::next_val(self)
    }
}

impl SplitMix<u128> {
    const ADD: u128 = 0x9e3779b9_7f4a7c15_243f6a88;
    const MU1: u128 = 0xbf58476d_1ce4e5b9_b7e15162;
    const MU2: u128 = 0x94d049bb_133111eb_9e3779b9;
    const SH1: usize = 60;
    const SH2: usize = 54;
    const SH3: usize = 62;

    pub fn next<V>(&mut self) -> V
    where
        Self: p128::Prng<V>,
    {
        p128::Prng::next_val(self)
    }
}

impl p32::Prng32 for SplitMix<u32> {
    fn calc(&mut self) -> u32 {
        self.state = self.state.wrapping_add(Self::ADD);
        let z = self.state;
        let z = (z ^ (z >> Self::SH1)).wrapping_mul(Self::MU1);
        let z = (z ^ (z >> Self::SH2)).wrapping_mul(Self::MU2);
        z ^ (z >> Self::SH3)
    }
}

impl p64::Prng64 for SplitMix<u64> {
    fn calc(&mut self) -> u64 {
        self.state = self.state.wrapping_add(Self::ADD);
        let z = self.state;
        let z = (z ^ (z >> Self::SH1)).wrapping_mul(Self::MU1);
        let z = (z ^ (z >> Self::SH2)).wrapping_mul(Self::MU2);
        z ^ (z >> Self::SH3)
    }
}

impl p128::Prng128 for SplitMix<u128> {
    fn calc(&mut self) -> u128 {
        self.state = self.state.wrapping_add(Self::ADD);
        let z = self.state;
        let z = (z ^ (z >> Self::SH1)).wrapping_mul(Self::MU1);
        let z = (z ^ (z >> Self::SH2)).wrapping_mul(Self::MU2);
        z ^ (z >> Self::SH3)
    }
}

pub type SplitMix32 = SplitMix<u32>;
pub type SplitMix64 = SplitMix<u64>;
pub type SplitMix128 = SplitMix<u128>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dump_value_types() {
        let mut rng = SplitMix::new(123456_u64);

        let a: u8 = rng.next();
        assert_eq!(a, 230);

        let b: u128 = rng.next();
        assert_eq!(b, 200611180297160101390585888926671092745);

        let c: bool = rng.next();
        assert_eq!(c, false);

        let d: f32 = rng.next();
        assert_eq!(d, 0.20730236);
    }
}
