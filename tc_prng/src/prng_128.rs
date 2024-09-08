use crate::Prng128 as Prng;

pub(crate) trait Prng128 {
    fn calc(&mut self) -> u128;
}

impl<T: Prng128> Prng<i8> for T {
    fn next(&mut self) -> i8 {
        let v = self.calc();
        (v >> 72) as i8
    }
}

impl<T: Prng128> Prng<i16> for T {
    fn next(&mut self) -> i16 {
        let v = self.calc();
        (v >> 64) as i16
    }
}

impl<T: Prng128> Prng<i32> for T {
    fn next(&mut self) -> i32 {
        let v = self.calc();
        (v >> 64) as i32
    }
}

impl<T: Prng128> Prng<i64> for T {
    fn next(&mut self) -> i64 {
        let v = self.calc();
        (v >> 32) as i64
    }
}

impl<T: Prng128> Prng<i128> for T {
    fn next(&mut self) -> i128 {
        let v = self.calc();
        v as i128
    }
}

impl<T: Prng128> Prng<u8> for T {
    fn next(&mut self) -> u8 {
        let v = self.calc();
        (v >> 72) as u8
    }
}

impl<T: Prng128> Prng<u16> for T {
    fn next(&mut self) -> u16 {
        let v = self.calc();
        (v >> 64) as u16
    }
}

impl<T: Prng128> Prng<u32> for T {
    fn next(&mut self) -> u32 {
        let v = self.calc();
        (v >> 64) as u32
    }
}

impl<T: Prng128> Prng<u64> for T {
    fn next(&mut self) -> u64 {
        let v = self.calc();
        (v >> 32) as u64
    }
}

impl<T: Prng128> Prng<u128> for T {
    fn next(&mut self) -> u128 {
        let v = self.calc();
        v
    }
}


impl<T: Prng128> Prng<f32> for T {
    fn next(&mut self) -> f32 {
        const MASK: u128 = (1 << 23) - 1;
        const D: f32 = MASK as f32;
        let n = ((self.calc() >> 106) & MASK) as f32;
        n / D
    }
}

impl<T: Prng128> Prng<f64> for T {
    fn next(&mut self) -> f64 {
        const MASK: u128 = (1 << 54) - 1;
        const D: f64 = MASK as f64;
        let n = ((self.calc() >> 74) & MASK) as f64;
        n / D
    }
}

impl<T: Prng128> Prng<bool> for T {
    fn next(&mut self) -> bool {
        let v = self.calc() & (1 << 64);
        v > 0
    }
}
