use crate::{
    allocator::Allocator,
    particle::{coord::Coordinate, Particle},
};

/// The algorithm to use for PSO
pub trait Algorithm<P, C, O, T>
where
    P: Particle<C>,
    C: Coordinate,
    O: Fn(&P) -> T,
    T: PartialOrd,
{
    fn search<'a, A>(&'a self, allocator: &mut A, objective_func: &O)
    where
        A: Allocator<P, C>,
        for<'b> &'b mut A: IntoIterator<Item = &'b mut P>,
        P: 'a;
}

// TODO: Try and implement this trait for function types

/// The canconical implementaion of the PSO algorithm
pub struct Canonical;

impl<P, C, O, T> Algorithm<P, C, O, T> for Canonical
where
    P: Particle<C>,
    C: Coordinate,
    O: Fn(&P) -> T,
    T: PartialOrd,
{
    fn search<'a, A>(&'a self, allocator: &mut A, objective_func: &O)
    where
        A: Allocator<P, C>,
        for<'b> &'b mut A: IntoIterator<Item = &'b mut P>,
        P: 'a,
    {
        // finding best particle
        // TODO: Consider changing the ordering that was returned if None is returned
        // TODO: Is panicking if a min is not found the best option?
        let best = allocator.into_iter();
    }
}
