pub mod coord;
pub mod vec_particle;

use self::coord::CoordinateElement;
use crate::allocator::Size;

/// This is a particle in PSO
/// In essence it is just a list of numbers that make up the "coordinates" of the particle
pub trait Particle<T>
where
    T: CoordinateElement,
{
    /// Creates a new Particle
    fn new(dims: impl Size) -> Self;

    /// Returns a reference to the coordinates
    fn coord(&self) -> &T;

    /// Returns a mutable reference to the coordinates
    fn coord_mut(&self) -> &mut T;

    /// Returns a reference to the velocity
    fn vec(&self) -> &T;

    /// Returns a mutable reference to the velocity
    fn vec_mut(&self) -> &mut T;
}
