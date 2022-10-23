pub struct SimpleRandom {
    multiplier: i64,
    mask: i64,
    seed: i64,
}

impl SimpleRandom {
    pub fn new() -> SimpleRandom {
        let multiplier: i64 = 25214903917;
        let mask: i64 = 281474976710655;
        let seed: i64 = 0;

        SimpleRandom {
            multiplier,
            mask,
            seed,
        }
    }

    pub fn next(&mut self) -> i32 {
        // utils::set_panic_hook();
        self.seed = (self.seed.wrapping_mul(self.multiplier) + 11) & self.mask;
        return (self.seed >> 17) as i32;
    }

    pub fn set_seed(&mut self, new_seed: i64) {
        self.seed = (new_seed ^ self.multiplier) & self.mask;
    }

    pub fn next_int(&mut self, bound: i32) -> i32 {
        let mut r: i32 = self.next();
        let m: i32 = bound - 1;
        if (bound & m) == 0 {
            r = ((bound as i64 * r as i64) >> 31) as i32;
        } else {
            let mut u: i32 = r;
            r = u % bound;
            while u - r + m < 0 {
                u = self.next();
                r = u % bound;
            }
        }
        return r;
    }
}
