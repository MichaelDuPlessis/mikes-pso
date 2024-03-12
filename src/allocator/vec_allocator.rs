use super::allocator::{Allocator, Size};
use crate::particle::particle::Particle;
use std::{marker::PhantomData, ops, vec::IntoIter};

// TODO: Give it a better name
/// This is the mos basic form of allocator
pub struct VecAllocator<P, I>
where
    P: Particle<I>,
{
    particles: Vec<P>,
    amount: usize,
    dims: usize,
    _particle_index: PhantomData<I>,
}

impl<P, I> ops::Index<usize> for VecAllocator<P, I>
where
    P: Particle<I>,
{
    type Output = P;

    fn index(&self, index: usize) -> &Self::Output {
        &self.particles[index]
    }
}

impl<P, I> IntoIterator for VecAllocator<P, I>
where
    P: Particle<I>,
{
    type Item = P;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.particles.into_iter()
    }
}

impl<P, I> Allocator<P, usize, I> for VecAllocator<P, I>
where
    P: Particle<I>,
{
    fn allocate(dims: impl Size, amount: impl Size) -> Self {
        let amount = amount.size();
        let particles = (0..amount).map(|_| P::new(dims)).collect();
        Self {
            particles,
            amount,
            dims: dims.size(),
            _particle_index: PhantomData,
        }
    }

    fn amount(&self) -> usize {
        self.amount
    }

    fn dims(&self) -> usize {
        self.dims
    }
}
