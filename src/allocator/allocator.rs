use crate::particle::particle::Particle;
use std::ops;

/// This trait is implemented by types that can give sizes
/// It is used for both dynamic and const sizes
/// and is directly inspired by the nalgebra crate
pub trait Size: Copy {
    fn size(&self) -> usize;
}

/// A constant size (known at compile time)
#[derive(Clone, Copy)]
pub struct ConstSize<const S: usize>;

impl<const S: usize> Size for ConstSize<S> {
    fn size(&self) -> usize {
        S
    }
}

impl<const S: usize> From<()> for ConstSize<S> {
    fn from(_: ()) -> Self {
        Self
    }
}

/// A dynamic size (known at runtime)
#[derive(Clone, Copy)]
pub struct DynSize(usize);

impl Size for DynSize {
    fn size(&self) -> usize {
        self.0
    }
}

impl From<usize> for DynSize {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

/// This trait is implemented by types that allocate particles
/// The type implementing the trait will most likely need to be able to store the particles it creats so that they can be indexed later
pub trait Allocator<P, I, J>: ops::Index<I> + IntoIterator<Item = P>
where
    P: Particle<J>,
{
    /// This method allocates the particles
    /// amount is the number of particles to be allocated
    fn allocate(dims: impl Size, amount: impl Size) -> Self;

    /// This method returns the dimensions of each particle
    fn amount(&self) -> usize;

    /// This mehtod returns the number of particles
    fn dims(&self) -> usize;
}
