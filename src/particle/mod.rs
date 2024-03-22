pub mod coord;
pub mod vec_particle;

use self::coord::Coordinate;
use crate::allocator::Size;

/// This is a particle in PSO
/// In essence it is just a list of numbers that make up the "coordinates" of the particle
// TODO: Make it so that vectors and coordinates do not need to be the same type
pub trait Particle<T>
where
    T: Coordinate,
{
    /// Creates a new Particle
    fn new(dims: impl Size) -> Self;

    /// Returns a reference to the coordinates
    fn coord(&self) -> &T;

    /// Returns a mutable reference to the coordinates
    fn coord_mut(&self) -> &mut T;

    /// Returns a reference to the velocity
    fn vel(&self) -> &T;

    /// Returns a mutable reference to the velocity
    fn vel_mut(&self) -> &mut T;
}
