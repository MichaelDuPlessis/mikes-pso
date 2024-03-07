use crate::particle::coord::CoordinateElement;
use std::ops;

/// This trait is implemented by types that can give dimensions
/// It is used for both dynamic and const dimensions
/// and is directly inspired by the nalgebra crate
pub trait Dim {
    fn value(&self) -> usize;
}

/// A constant sized dimension (known at compile time)
pub struct Const<const D: usize>;

impl<const D: usize> Dim for Const<D> {
    fn value(&self) -> usize {
        D
    }
}

/// A dynamic sized dimension (known at runtime)
pub struct Dyn(usize);

impl Dim for Dyn {
    fn value(&self) -> usize {
        self.0
    }
}

/// This trait is implemented by types that allocate particles
/// The type implementing the trait will most likely need to be able to store the particles it creats so that they can be indexed later
pub trait ParticleAllocator<C, I>: ops::Index<usize> + IntoIterator<Item = C>
where
    C: CoordinateElement,
{
    /// This method allocates the particles
    /// amount is the number of particles to be allocated
    fn allocate(amount: usize) -> Self;
}
