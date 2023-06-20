use crate::{ObjectiveFunction, VelocityFunction};
use rand::Rng;

// represents a particle
#[derive(Clone)]
pub struct Particle<const DIMS: usize> {
    coordinates: [f64; DIMS],
    best: [f64; DIMS],
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
            best: [f64::MAX; DIMS],
            velocity: [0.0; DIMS],
        }
    }

    pub fn apply_function<F>(&mut self, func: &F) -> f64
    where
        F: ObjectiveFunction<DIMS>,
    {
        let res = func(&self.coordinates);
        self.best = if res < func(&self.best) {
            self.coordinates
        } else {
            self.best
        };
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

    pub fn velocity(&self) -> [f64; DIMS] {
        self.velocity
    }

    pub fn best(&self) -> [f64; DIMS] {
        self.best
    }

    pub fn coordinates(&self) -> [f64; DIMS] {
        self.coordinates
    }
}
