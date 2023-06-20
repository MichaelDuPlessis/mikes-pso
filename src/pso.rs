use crate::{particle::Particle, ObjectiveFunction, VelocityFunction};
use std::error::Error;

pub struct PSO<V, const DIMS: usize>
where
    V: VelocityFunction<DIMS>,
{
    particles: Vec<Particle<DIMS>>,
    n_particles: usize,
    bounds: [(f64, f64); DIMS],
    rng: rand::rngs::ThreadRng,
    velocity: V,
}

impl<V, const DIMS: usize> PSO<V, DIMS>
where
    V: VelocityFunction<DIMS>,
{
    pub fn new(
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

    fn optimize<F>(&mut self, generations: usize, obj_func: F) -> Particle<DIMS>
    where
        F: ObjectiveFunction<DIMS>,
    {
        // creating random particles
        self.generate_random_particles();

        // adding best position of every particle
        self.particles.iter_mut().for_each(|p| {
            p.apply_function(&obj_func);
        });

        // getting global best
        let mut g_best = self
            .particles
            .iter()
            .min_by(|p1, p2| obj_func(&p1.coordinates()).total_cmp(&obj_func(&p2.coordinates())))
            .unwrap()
            .clone(); // sadly the clone is necessary

        for _ in 0..generations {
            for p in &mut self.particles {
                p.apply_velocity(&g_best, &self.velocity);
            }

            g_best = self
                .particles
                .iter()
                .min_by(|p1, p2| {
                    obj_func(&p1.coordinates()).total_cmp(&obj_func(&p2.coordinates()))
                })
                .unwrap()
                .clone(); // sadly the clon // sadly the clone is necessary
        }

        g_best
    }
}
