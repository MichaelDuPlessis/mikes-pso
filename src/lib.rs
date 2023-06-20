mod params;
mod particle;

use params::Params;
use particle::Particle;
use rand::Rng;
use std::error::Error;

struct PSO<V, const DIMS: usize>
where
    V: Fn(&Particle<DIMS>, &rand::rngs::ThreadRng) -> [f64; DIMS],
{
    particles: Vec<Particle<DIMS>>,
    n_particles: usize,
    bounds: [(f64, f64); DIMS],
    rng: rand::rngs::ThreadRng,
    velocity: V,
}

impl<V, const DIMS: usize> PSO<V, DIMS>
where
    V: Fn(&Particle<DIMS>, &rand::rngs::ThreadRng) -> [f64; DIMS],
{
    fn new(
        n_particles: usize,
        bounds: [(f64, f64); DIMS],
        velocity: V,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            particles: Vec::with_capacity(n_particles),
            n_particles,
            bounds,
            rng: rand::thread_rng(),
            velocity,
        })
    }

    // initializes all the particles with random values
    fn generate_random_particles(&mut self) {
        for _ in 0..self.n_particles {
            self.particles
                .push(Particle::new(&self.bounds, &mut self.rng))
        }
    }

    fn optimize<F>(&mut self, generations: usize, target_func: F) -> &Particle<DIMS>
    where
        F: Fn(&[f64; DIMS]) -> f64,
    {
        // creating random particles
        self.generate_random_particles();

        // adding best position of every particle
        self.particles.iter_mut().for_each(|p| {
            p.apply_function(&target_func);
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
