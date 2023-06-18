mod particle;

use particle::Particle;
use rand::Rng;
use std::error::Error;

struct PSO<const DIMS: usize> {
    particles: Vec<Particle<DIMS>>,
    n_particles: usize,
    bounds: [(f64, f64); DIMS],
    w: f64,
    c1: f64,
    c2: f64,
    rng: rand::rngs::ThreadRng,
}

impl<const DIMS: usize> PSO<DIMS> {
    fn new(
        n_particles: usize,
        bounds: [(f64, f64); DIMS],
        w: f64,
        c1: f64,
        c2: f64,
    ) -> Result<Self, Box<dyn Error>> {
        if w > 1.0 || w < 0.0 {
            return Err(Box::from(
                "The inertia weight constant must be between 0 and 1.",
            ));
        }

        if bounds.iter().any(|(l, u)| l > u) {
            return Err(Box::from(
                "The lower bound must be smaller then upper bound.",
            ));
        }

        let mut rng = rand::thread_rng();
        Ok(Self {
            particles: Vec::with_capacity(n_particles),
            n_particles,
            bounds,
            w,
            c1,
            c2,
            rng: rand::thread_rng(),
        })
    }

    // initializes all the particles with random values
    fn generate_random_particles(&mut self) {
        for _ in 0..self.n_particles {
            self.particles
                .push(Particle::new(&self.bounds, &mut self.rng))
        }
    }

    fn optimize<F>(&mut self, generations: usize, func: &F) -> &Particle<DIMS>
    where
        F: Fn(&[f64]) -> f64,
    {
        // creating random particles
        self.generate_random_particles();

        // adding best position of every particle
        self.particles.iter_mut().for_each(|p| {
            p.apply_function(func);
        });

        // getting global best
        let g_best = self
            .particles
            .iter()
            .min_by(|p1, p2| Particle::compare(p1, p2))
            .unwrap();

        for _ in 0..generations {
            for p in &self.particles {
                let (r1, r2): (f64, f64) = self.rng.gen();
            }
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction_test() {}
}
