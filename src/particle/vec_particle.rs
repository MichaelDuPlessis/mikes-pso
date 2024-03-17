use crate::allocator::Size;
use rand::distributions::{Distribution, Uniform};
use std::ops::{self, AddAssign};

use super::Particle;

/// This is a particle in PSO
pub struct VecParticle<T>(Vec<T>)
where
    T: ops::AddAssign;

impl<T> ops::Add for VecParticle<T>
where
    T: AddAssign,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // getting values inside Particle
        let Self(mut lhs) = self;
        let Self(rhs) = rhs;

        // adding rhs to lhs
        for (l, r) in lhs.iter_mut().zip(rhs) {
            *l += r;
        }

        Self(lhs)
    }
}

impl<T> ops::AddAssign for VecParticle<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        let Self(lhs) = self;
        let Self(rhs) = rhs;

        for (l, r) in lhs.iter_mut().zip(rhs) {
            *l += r;
        }
    }
}

impl Particle<f32> for VecParticle<f32> {
    fn new(dims: impl Size) -> Self {
        let mut rng = rand::thread_rng();
        let between = Uniform::new_inclusive(f32::MIN, f32::MAX);

        let particles = (0..dims.size()).map(|_| between.sample(&mut rng)).collect();

        Self(particles)
    }

    fn coord(&self) -> &f32 {
        todo!()
    }

    fn coord_mut(&self) -> &mut f32 {
        todo!()
    }

    fn vec(&self) -> &f32 {
        todo!()
    }

    fn vec_mut(&self) -> &mut f32 {
        todo!()
    }
}
