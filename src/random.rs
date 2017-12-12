extern crate rand;

use self::rand::{ Rng, StdRng, SeedableRng };

pub struct Random {
    rng: StdRng,
}

impl Random {
    pub fn from_seed(seed: usize) -> Random {
        let rand_seed: &[_] = &[seed];
        Random {
            rng: SeedableRng::from_seed(rand_seed),
        }
    }

    pub fn new() -> Random {
        let seed = rand::random::<usize>();
        Random::from_seed(seed)
    }
    
    pub fn reseed(&mut self, seed: usize) {
        let rand_seed: &[_] = &[seed];
        self.rng.reseed(rand_seed);
    }

    pub fn float(&mut self, min: f64, max: f64) -> f64 {
        self.rng.gen_range(min, max)
    }

    pub fn int(&mut self, min: i32, max: i32) -> i32 {
        self.rng.gen_range(min, max)
    }

    pub fn boolean(&mut self) -> bool {
        self.weighted_bool(0.5)
    }

    pub fn weighted_bool(&mut self, weight: f64) -> bool {
        self.float(0.0, 1.0) < weight
    }

}
