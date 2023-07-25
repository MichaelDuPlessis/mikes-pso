use std::fmt::Debug;

use crate::{particle::Particle, ObjectiveFunction, VelocityFunction};

pub struct PSO<V, const DIMS: usize>
where
    V: VelocityFunction<DIMS>,
{
    particles: Vec<Particle<DIMS>>,
    n_particles: usize,
    bounds: [(f64, f64); DIMS],
    velocity: V,
}

impl<V, const DIMS: usize> Debug for PSO<V, DIMS>
where
    V: VelocityFunction<DIMS>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PSO")
            .field("particles", &self.particles)
            .field("n_particles", &self.n_particles)
            .field("bounds", &self.bounds)
            .finish()
    }
}

impl<V, const DIMS: usize> PSO<V, DIMS>
where
    V: VelocityFunction<DIMS>,
{
    pub fn new(n_particles: usize, bounds: [(f64, f64); DIMS], velocity: V) -> Self {
        Self {
            particles: Vec::with_capacity(n_particles),
            n_particles,
            bounds,
            velocity,
        }
    }

    pub fn optimize<F>(&mut self, generations: usize, obj_func: F) -> Particle<DIMS>
    where
        F: ObjectiveFunction<DIMS>,
    {
        // creating random particles
        for _ in 0..self.n_particles {
            self.particles.push(Particle::new(&self.bounds))
        }

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
                .clone(); // sadly the clone is necessary
        }

        g_best
    }
}
