use crate::Prng64 as Prng;

pub(crate) trait Prng64 {
    fn calc(&mut self) -> u64;
}

impl<T: Prng64> Prng<i8> for T {
    fn next(&mut self) -> i8 {
        let v = self.calc();
        (v >> 48) as i8
    }
}

impl<T: Prng64> Prng<i16> for T {
    fn next(&mut self) -> i16 {
        let v = self.calc();
        (v >> 37) as i16
    }
}

impl<T: Prng64> Prng<i32> for T {
    fn next(&mut self) -> i32 {
        let v = self.calc();
        (v >> 16) as i32
    }
}

impl<T: Prng64> Prng<i64> for T {
    fn next(&mut self) -> i64 {
        let v = self.calc();
        v as i64
    }
}

impl<T: Prng64> Prng<i128> for T {
    fn next(&mut self) -> i128 {
        let l = self.calc() as u128;
        let h = self.calc() as u128;
        ((h << 64) | l) as i128
    }
}

impl<T: Prng64> Prng<u8> for T {
    fn next(&mut self) -> u8 {
        let v = self.calc();
        (v >> 48) as u8
    }
}

impl<T: Prng64> Prng<u16> for T {
    fn next(&mut self) -> u16 {
        let v = self.calc();
        (v >> 37) as u16
    }
}

impl<T: Prng64> Prng<u32> for T {
    fn next(&mut self) -> u32 {
        let v = self.calc();
        (v >> 16) as u32
    }
}

impl<T: Prng64> Prng<u64> for T {
    fn next(&mut self) -> u64 {
        self.calc()
    }
}

impl<T: Prng64> Prng<u128> for T {
    fn next(&mut self) -> u128 {
        let l = self.calc() as u128;
        let h = self.calc() as u128;
        (h << 64) | l
    }
}

impl<T: Prng64> Prng<f32> for T {
    fn next(&mut self) -> f32 {
        const MASK: u64 = (1 << 23) - 1;
        const D: f32 = MASK as f32;
        let n = ((self.calc() >> 42) & MASK) as f32;
        n / D
    }
}

impl<T: Prng64> Prng<f64> for T {
    fn next(&mut self) -> f64 {
        const MASK: u64 = (1 << 54) - 1;
        const D: f64 = MASK as f64;
        let n = ((self.calc() >> 10) & MASK) as f64;
        n / D
    }
}

impl<T: Prng64> Prng<bool> for T {
    fn next(&mut self) -> bool {
        let v = self.calc() & (1 << 32);
        v > 0
    }
}
