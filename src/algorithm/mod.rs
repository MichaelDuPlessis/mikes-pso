use std::{marker::PhantomData, ops};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

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
    O: Fn(&C) -> T,
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
pub struct Canonical<C, W, C1, C2, R1, R2>
where
    W: ops::Mul<C, Output = C> + Copy,
    C1: ops::Mul<C, Output = C> + Copy,
    C2: ops::Mul<C, Output = C> + Copy,
    R1: ops::Mul<C, Output = C> + Copy,
    R2: ops::Mul<C, Output = C> + Copy,
    Standard: Distribution<R1>,
    Standard: Distribution<R2>,
{
    w: W,
    c1: C1,
    c2: C2,
    _coordinate: PhantomData<C>,
    _rand1: PhantomData<R1>,
    _rand2: PhantomData<R2>,
}

impl<C, W, C1, C2, R1, R2> Canonical<C, W, C1, C2, R1, R2>
where
    W: ops::Mul<C, Output = C> + Copy,
    C1: ops::Mul<C, Output = C> + Copy,
    C2: ops::Mul<C, Output = C> + Copy,
    R1: ops::Mul<C, Output = C> + Copy,
    R2: ops::Mul<C, Output = C> + Copy,
    Standard: Distribution<R1>,
    Standard: Distribution<R2>,
{
    /// Create a new canonical pso algorithm with specified intertia weght, cognitive and social coeffients
    pub fn new(w: W, c1: C1, c2: C2) -> Self {
        Self {
            w,
            c1,
            c2,
            _coordinate: PhantomData,
            _rand1: PhantomData,
            _rand2: PhantomData,
        }
    }
}

impl<P, C, O, T, W, C1, C2, R1, R2> Algorithm<P, C, O, T> for Canonical<C, W, C1, C2, R1, R2>
where
    P: Particle<C>,
    C: Coordinate,
    O: Fn(&C) -> T,
    T: PartialOrd,
    W: ops::Mul<C, Output = C> + Copy,
    C1: ops::Mul<C, Output = C> + Copy,
    C2: ops::Mul<C, Output = C> + Copy,
    R1: ops::Mul<C, Output = C> + Copy,
    R2: ops::Mul<C, Output = C> + Copy,
    Standard: Distribution<R1>,
    Standard: Distribution<R2>,
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

        // Sadly a clone is necessary
        let best = allocator
            .into_iter()
            .min_by(|x, y| {
                objective_func(x.coord())
                    .partial_cmp(&objective_func(y.coord()))
                    .unwrap()
            })
            .unwrap()
            .clone();

        // TODO: currently every particle is run through the objective function twice this should be fixed
        allocator.into_iter().for_each(|p| {
            // getting new velocity
            let vel = self.w * *p.vel()
                + self.c1 * (rand::thread_rng().gen::<R1>() * (*p.best_coord() - *p.coord()))
                + self.c2 * (rand::thread_rng().gen::<R2>() * (*best.best_coord() - *p.coord()));

            // updaing old velocity
            *p.vel_mut() = vel;

            // updaing coordinates
            *p.coord_mut() = *p.coord() + vel;

            if let Some(std::cmp::Ordering::Greater) =
                objective_func(p.best_coord()).partial_cmp(&objective_func(p.coord()))
            {
                *p.best_coord_mut() = *p.coord();
            }
        });
    }
}
