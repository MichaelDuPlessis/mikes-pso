mod algorithm;
mod allocator;
mod particle;

use algorithm::Algorithm;
use allocator::Allocator;
use particle::{coord::CoordinateElement, Particle};
use std::marker::PhantomData;

/// This is a generic PSO struct
/// It is the main export of this library and is what is used to run the PSO
pub struct PSO<P, A, F, O, T, K>
where
    A: Allocator<P, K>,
    for<'a> &'a mut A: IntoIterator<Item = &'a mut P>,
    P: Particle<K>,
    F: Algorithm<P, K, O, T>,
    O: Fn(&P) -> T,
    T: PartialOrd,
    K: CoordinateElement,
{
    allocator: A,
    algorithm: F,
    objective_func: O,
    iterations: usize,
    _particle: PhantomData<P>,
    _element: PhantomData<K>,
}

impl<F, P, A, O, T, K> PSO<P, A, F, O, T, K>
where
    A: Allocator<P, K>,
    for<'a> &'a mut A: IntoIterator<Item = &'a mut P>,
    P: Particle<K>,
    F: Algorithm<P, K, O, T>,
    O: Fn(&P) -> T,
    T: PartialOrd,
    K: CoordinateElement,
{
    /// Creates a new PSO from an allocator
    pub fn new(allocator: A, algorithm: F, iterations: usize, objective_func: O) -> Self {
        Self {
            allocator,
            algorithm,
            objective_func,
            iterations,
            _particle: PhantomData,
            _element: PhantomData,
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
            self.algorithm
                .search(&mut self.allocator, &self.objective_func);
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
