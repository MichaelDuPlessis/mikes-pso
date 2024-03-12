mod algorithm;
mod allocator;
mod particle;

use allocator::allocator::{Allocator, Size};
use particle::particle::Particle;
use std::marker::PhantomData;

/// This is a generic PSO struct
/// It is the main export of this library and is what is used to run the PSO
struct PSO<P, I, J, A>
where
    A: Allocator<P, I, J>,
    P: Particle<J>,
{
    allocator: A,
    _particle: PhantomData<P>,
    _allocator_index: PhantomData<I>,
    _particle_index: PhantomData<J>,
}

impl<P, I, J, A> PSO<P, I, J, A>
where
    A: Allocator<P, I, J>,
    P: Particle<J>,
{
    /// Creates a new PSO from an allocator
    pub fn new(allocator: A) -> Self {
        Self {
            allocator,
            _particle: PhantomData,
            _allocator_index: PhantomData,
            _particle_index: PhantomData,
        }
    }

    /// Returns the number of dimensions of a particle
    pub fn dims(&self) -> usize {
        self.allocator.dims()
    }

    /// Returns the amount of particles
    pub fn amount(&self) -> usize {
        self.allocator.amount()
    }

    /// Tries to find the minimum of a function, returns the particle closest to the minimum
    pub fn find_min(&self) -> P {
        todo!()
    }

    /// Tries to find the maximum of a function, returns the particle closest to the maximum
    pub fn find_max(&self) -> P {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
