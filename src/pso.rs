use crate::{particle::Particle, ObjectiveFunction, VelocityFunction};
use std::fmt::Debug;

pub struct PSO<V, F, const DIMS: usize>
where
    V: VelocityFunction<DIMS>,
    F: ObjectiveFunction<DIMS>,
{
    particles: Vec<Particle<DIMS>>,
    n_particles: usize,
    generations: usize,
    bounds: [(f64, f64); DIMS],
    velocity: V,
    objective: F,
}

impl<V, F, const DIMS: usize> Debug for PSO<V, F, DIMS>
where
    V: VelocityFunction<DIMS>,
    F: ObjectiveFunction<DIMS>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PSO")
            .field("particles", &self.particles)
            .field("n_particles", &self.n_particles)
            .field("bounds", &self.bounds)
            .finish()
    }
}

impl<V, F, const DIMS: usize> PSO<V, F, DIMS>
where
    V: VelocityFunction<DIMS>,
    F: ObjectiveFunction<DIMS>,
{
    pub fn new(
        n_particles: usize,
        generations: usize,
        bounds: [(f64, f64); DIMS],
        velocity: V,
        objective: F,
    ) -> Self {
        Self {
            particles: Vec::with_capacity(n_particles),
            n_particles,
            generations,
            bounds,
            velocity,
            objective,
        }
    }

    pub fn optimize(&mut self) -> Particle<DIMS> {
        // creating random particles
        for _ in 0..self.n_particles {
            self.particles.push(Particle::new_random(&self.bounds))
        }

        // adding best position of every particle
        self.particles.iter_mut().for_each(|p| {
            p.apply_function(&self.objective);
        });

        // getting global best
        let mut g_best = self
            .particles
            .iter()
            .min_by(|p1, p2| {
                (self.objective)(&p1.coordinates()).total_cmp(&(self.objective)(&p2.coordinates()))
            })
            .unwrap()
            .clone(); // sadly the clone is necessary

        for _ in 0..self.generations {
            for p in &mut self.particles {
                p.apply_velocity(&g_best, &self.velocity);
            }

            g_best = self
                .particles
                .iter()
                .min_by(|p1, p2| {
                    (self.objective)(&p1.coordinates())
                        .total_cmp(&(self.objective)(&p2.coordinates()))
                })
                .unwrap()
                .clone(); // sadly the clone is necessary
        }

        g_best
    }
}
