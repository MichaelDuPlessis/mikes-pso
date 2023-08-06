pub mod particle;
pub mod pso;
pub mod vector;

use particle::Particle;
use vector::Vector;

// using it as a trait bound alias
// particle should contain all the needed information
pub trait VelocityFunction<const DIMS: usize>: Fn(/*current*/ &Particle<DIMS>, /*best*/ &Particle<DIMS>) -> Vector<DIMS> {}
impl<F, const DIMS: usize> VelocityFunction<DIMS> for F where
    F: Fn(&Particle<DIMS>, &Particle<DIMS>) -> Vector<DIMS>
{
}

pub trait ObjectiveFunction<const DIMS: usize>: Fn(&Vector<DIMS>) -> f64 {}
impl<F, const DIMS: usize> ObjectiveFunction<DIMS> for F where F: Fn(&Vector<DIMS>) -> f64 {}

pub fn velocity<const DIMS: usize>(
    current: &Particle<DIMS>,
    best: &Particle<DIMS>,
) -> Vector<DIMS> {
    let c1 = 1.0;
    let c2 = 1.0;
    let w = 0.5;
    let (r1, r2): (f64, f64) = rand::random();

    let vel = w * current.velocity()
        + c1 * r1 * (best.coordinates() - current.coordinates())
        + c2 * r2 * (current.best() - current.coordinates());

    vel
}
