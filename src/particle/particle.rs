use crate::allocator::allocator::Size;
use rand::distributions::{Distribution, Uniform};
use std::ops::{self, AddAssign};

/// This is a particle in PSO
/// In essence it is just a list of numbers that make up the "coordinates" of the particle
/// It is a marker trait
pub trait Particle<I>: ops::Add + ops::AddAssign + ops::Index<I> + Sized {
    /// Creates a new particle with random values for the coordinates
    fn new(dims: impl Size) -> Self;
}

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

impl<T> ops::Index<usize> for VecParticle<T>
where
    T: ops::AddAssign,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Particle<usize> for VecParticle<f32> {
    fn new(dims: impl Size) -> Self {
        let mut rng = rand::thread_rng();
        let between = Uniform::new_inclusive(f32::MIN, f32::MAX);

        let particles = (0..dims.size()).map(|_| between.sample(&mut rng)).collect();

        Self(particles)
    }
}
