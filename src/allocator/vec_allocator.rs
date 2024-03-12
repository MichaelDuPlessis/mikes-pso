use super::allocator::{Allocator, Size};
use crate::particle::particle::Particle;

// TODO: Give it a better name
/// This is the mos basic form of allocator
pub struct VecAllocator<P>
where
    P: Particle,
{
    particles: Vec<P>,
    amount: usize,
    dims: usize,
}

impl<P> Allocator<P> for VecAllocator<P>
where
    P: Particle,
{
    fn allocate(dims: impl Size, amount: impl Size) -> Self {
        let amount = amount.size();
        let particles = (0..amount).map(|_| P::new(dims)).collect();
        Self {
            particles,
            amount,
            dims: dims.size(),
        }
    }

    fn amount(&self) -> usize {
        self.amount
    }

    fn dims(&self) -> usize {
        self.dims
    }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut P>
    where
        P: 'a,
    {
        self.particles.iter_mut()
    }
}
