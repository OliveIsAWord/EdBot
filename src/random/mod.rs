macro_rules! next_rng_for {
    ($int:ty, $name:ident) => {
        fn $name(&mut self) -> $int {
            let mut accum = 0;
            for _ in 0..<$int>::BITS {
                accum <<= 1;
                accum += self.next() as $int;
            }
            accum
        }
    };
}

pub trait Rng {
    fn next(&mut self) -> bool;

    next_rng_for!(u8, next8);
    next_rng_for!(u16, next16);
    next_rng_for!(u32, next32);
    next_rng_for!(u64, next64);
    next_rng_for!(u128, next128);
    next_rng_for!(usize, next_usize);

    fn sample<'a, T: std::fmt::Debug>(&mut self, vals: &'a [T]) -> &'a T {
        if vals.is_empty() {
            panic!("Can't choose element from empty collection.");
        }
        &vals[self.next_usize() % vals.len()]
    }

    fn get_probability(&mut self) -> f64 {
        self.next32() as f64 / u32::MAX as f64
    }
    fn run_probability(&mut self, p: f64) -> bool {
        p > self.get_probability()
    }
}

pub struct XorShift32(pub u32);

impl Rng for XorShift32 {
    fn next(&mut self) -> bool {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x & 1 != 0
    }
}
