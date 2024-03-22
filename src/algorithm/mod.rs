use std::{marker::PhantomData, ops};

use crate::{
    allocator::Allocator,
    particle::{coord::Coordinate, Particle},
};

/// The algorithm to use for PSO
// TODO: Try and implement this trait for function types
pub trait Algorithm<P, C, O, T>
where
    P: Particle<C>,
    C: Coordinate,
    O: Fn(&P) -> T,
    T: PartialOrd,
{
    /// Takes in an allocator and an objectve function and updates every particle
    // TODO: Try and have it something more generic than an allocator
    fn search<'a, A>(&'a self, allocator: &mut A, objective_func: &O)
    where
        A: Allocator<P, C>,
        for<'b> &'b mut A: IntoIterator<Item = &'b mut P>,
        P: 'a;
}

/// The canconical implementaion of the PSO algorithm
pub struct Canonical<C, W, C1, C2>
where
    W: ops::Mul<C, Output = C> + Copy,
    C1: ops::Mul<C, Output = C> + Copy,
    C2: ops::Mul<C, Output = C> + Copy,
{
    w: W,
    c1: C1,
    c2: C2,
    _coordinate: PhantomData<C>,
}

impl<C, W, C1, C2> Canonical<C, W, C1, C2>
where
    W: ops::Mul<C, Output = C> + Copy,
    C1: ops::Mul<C, Output = C> + Copy,
    C2: ops::Mul<C, Output = C> + Copy,
{
    /// Create a new canonical pso algorithm with specified intertia weght, cognitive and social coeffients
    pub fn new(w: W, c1: C1, c2: C2) -> Self {
        Self {
            w,
            c1,
            c2,
            _coordinate: PhantomData,
        }
    }
}

impl<P, C, O, T, W, C1, C2> Algorithm<P, C, O, T> for Canonical<C, W, C1, C2>
where
    P: Particle<C>,
    C: Coordinate,
    O: Fn(&P) -> T,
    T: PartialOrd,
    W: ops::Mul<C, Output = C> + Copy,
    C1: ops::Mul<C, Output = C> + Copy,
    C2: ops::Mul<C, Output = C> + Copy,
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
        let best = allocator
            .into_iter()
            .min_by(|x, y| objective_func(x).partial_cmp(&objective_func(y)).unwrap())
            .unwrap();

        allocator
            .into_iter()
            .for_each(|p| *p.coord_mut() = self.w * *p.vel());
    }
}
