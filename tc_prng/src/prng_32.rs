pub trait Prng<T> {
    fn next_val(&mut self) -> T;
}

pub(crate) trait Prng32 {
    fn calc(&mut self) -> u32;
}

impl<T: Prng32> Prng<i8> for T {
    fn next_val(&mut self) -> i8 {
        let v = self.calc();
        (v >> 24) as i8
    }
}

impl<T: Prng32> Prng<i16> for T {
    fn next_val(&mut self) -> i16 {
        let v = self.calc();
        (v >> 16) as i16
    }
}

impl<T: Prng32> Prng<i32> for T {
    fn next_val(&mut self) -> i32 {
        let v = self.calc();
        v as i32
    }
}

impl<T: Prng32> Prng<i64> for T {
    fn next_val(&mut self) -> i64 {
        let l = self.calc() as u64;
        let h = self.calc() as u64;
        ((h << 32) | l) as i64
    }
}

impl<T: Prng32> Prng<i128> for T {
    fn next_val(&mut self) -> i128 {
        let ll = self.calc() as u128;
        let lh = self.calc() as u128;
        let hl = self.calc() as u128;
        let hh = self.calc() as u128;
        ((hh << 96) | (hl << 64) | (lh << 32) | ll) as i128
    }
}

impl<T: Prng32> Prng<u8> for T {
    fn next_val(&mut self) -> u8 {
        let v = self.calc();
        (v >> 24) as u8
    }
}

impl<T: Prng32> Prng<u16> for T {
    fn next_val(&mut self) -> u16 {
        let v = self.calc();
        (v >> 16) as u16
    }
}

impl<T: Prng32> Prng<u32> for T {
    fn next_val(&mut self) -> u32 {
        let v = self.calc();
        v as u32
    }
}

impl<T: Prng32> Prng<u64> for T {
    fn next_val(&mut self) -> u64 {
        let l = self.calc() as u64;
        let h = self.calc() as u64;
        (h << 32) | l
    }
}

impl<T: Prng32> Prng<u128> for T {
    fn next_val(&mut self) -> u128 {
        let ll = self.calc() as u128;
        let lh = self.calc() as u128;
        let hl = self.calc() as u128;
        let hh = self.calc() as u128;
        (hh << 96) | (hl << 64) | (lh << 32) | ll
    }
}

impl<T: Prng32> Prng<f32> for T {
    fn next_val(&mut self) -> f32 {
        const MASK: u32 = (1 << 23) - 1;
        const D: f32 = MASK as f32;
        let n = ((self.calc() >> 10) & MASK) as f32;
        n / D
    }
}

impl<T: Prng32> Prng<f64> for T {
    fn next_val(&mut self) -> f64 {
        const MASK: u32 = (1 << 23) - 1;
        const D: f64 = MASK as f64;
        let n = ((self.calc() >> 10) & MASK) as f64;
        n / D
    }
}

impl<T: Prng32> Prng<bool> for T {
    fn next_val(&mut self) -> bool {
        let v = self.calc() & (1 << 16);
        v > 0
    }
}
