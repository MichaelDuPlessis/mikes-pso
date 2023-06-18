use std::cmp::Ordering;

use rand::Rng;

// represents a particle
pub struct Particle<const DIMS: usize> {
    dimensions: [f64; DIMS],
    best: f64,
    velocity: f64,
}

impl<const DIMS: usize> Particle<DIMS> {
    pub fn new(bounds: &[(f64, f64)], rng: &mut rand::rngs::ThreadRng) -> Self {
        let mut dimensions = [0.0; DIMS];

        for i in 0..dimensions.len() {
            let (lower, upper) = bounds[i];
            dimensions[i] = rng.gen_range(lower..=upper);
        }

        Self {
            dimensions,
            best: f64::MAX,
            velocity: 0.0,
        }
    }

    pub fn apply_function<F>(&mut self, func: &F) -> f64
    where
        F: Fn(&[f64]) -> f64,
    {
        let res = func(&self.dimensions);
        self.best = self.best.min(res);
        res
    }

    pub fn compare(p1: &Self, p2: &Self) -> Ordering {
        p1.best.total_cmp(&p2.best)
    }
}
