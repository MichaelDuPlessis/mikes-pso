use super::{Allocator, Size};
use crate::particle::{coord::Coordinate, Particle};
use std::{marker::PhantomData, slice::IterMut};

// TODO: Give it a better name
/// This is the mos basic form of allocator
pub struct VecAllocator<P, T>
where
    P: Particle<T>,
    T: Coordinate,
{
    particles: Vec<P>,
    amount: usize,
    dims: usize,
    _element: PhantomData<T>,
}

impl<'a, P, T> IntoIterator for &'a mut VecAllocator<P, T>
where
    P: Particle<T>,
    T: Coordinate,
{
    type Item = &'a mut P;

    type IntoIter = IterMut<'a, P>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl<P, T> Allocator<P, T> for VecAllocator<P, T>
where
    P: Particle<T>,
    T: Coordinate,
{
    fn allocate(dims: impl Size, amount: impl Size) -> Self {
        let amount = amount.size();
        let particles = (0..amount).map(|_| P::new(dims)).collect();
        Self {
            particles,
            amount,
            dims: dims.size(),
            _element: PhantomData,
        }
    }

    fn amount(&self) -> usize {
        self.amount
    }

    fn dims(&self) -> usize {
        self.dims
    }
}
