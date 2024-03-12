mod algorithm;
mod allocator;
mod particle;

use algorithm::Algorithm;
use allocator::allocator::{Allocator, Size};
use particle::particle::Particle;
use std::marker::PhantomData;

/// This is a generic PSO struct
/// It is the main export of this library and is what is used to run the PSO
pub struct PSO<P, A, F, O, T>
where
    A: Allocator<P>,
    P: Particle,
    F: Algorithm<P>,
    O: Fn(P) -> T,
{
    allocator: A,
    algorithm: F,
    objective_func: O,
    iterations: usize,
    _particle: PhantomData<P>,
}

impl<F, P, A, O, T> PSO<P, A, F, O, T>
where
    A: Allocator<P>,
    P: Particle,
    F: Algorithm<P>,
    O: Fn(P) -> T,
{
    /// Creates a new PSO from an allocator
    pub fn new(allocator: A, algorithm: F, iterations: usize, objective_func: O) -> Self {
        Self {
            allocator,
            algorithm,
            objective_func,
            iterations,
            _particle: PhantomData,
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
    pub fn find_min(&mut self) -> P {
        for _ in 0..self.iterations {
            self.algorithm.search(self.allocator.iter_mut());
        }

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
