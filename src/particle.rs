use crate::pso::{ObjectiveFunction, VelocityFunction};
use rand::Rng;
use std::cmp::Ordering;

// represents a particle
#[derive(Clone)]
pub struct Particle<const DIMS: usize> {
    coordinates: [f64; DIMS],
    best: f64,
    velocity: [f64; DIMS],
}

impl<const DIMS: usize> Particle<DIMS> {
    // creating a new particle withing the bounds of the objective
    pub fn new(bounds: &[(f64, f64)], rng: &mut rand::rngs::ThreadRng) -> Self {
        let mut coordinates = [0.0; DIMS];

        for i in 0..coordinates.len() {
            let (lower, upper) = bounds[i];
            coordinates[i] = rng.gen_range(lower..=upper);
        }

        Self {
            coordinates,
            best: f64::MAX,
            velocity: [0.0; DIMS],
        }
    }

    pub fn apply_function<F>(&mut self, func: &F) -> f64
    where
        F: ObjectiveFunction<DIMS>,
    {
        let res = func(&self.coordinates);
        self.best = self.best.min(res);
        res
    }

    pub fn apply_velocity<V>(&mut self, best: &Self, func: &V)
    where
        V: VelocityFunction<DIMS>,
    {
        let vel = func(self, best);

        for i in 0..self.coordinates.len() {
            // gives performance gain by removing bounds check
            unsafe {
                *self.coordinates.get_unchecked_mut(i) += vel.get_unchecked(i);
            }
        }
        self.velocity = vel;
    }

    pub fn compare(p1: &Self, p2: &Self) -> Ordering {
        p1.best.total_cmp(&p2.best)
    }
}
