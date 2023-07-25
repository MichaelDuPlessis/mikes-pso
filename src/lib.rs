pub mod particle;
pub mod pso;

use particle::Particle;

// using it as a trait bound alias
// particle should contain all the needed information
pub trait VelocityFunction<const DIMS: usize>: Fn(/*current*/ &Particle<DIMS>, /*best*/ &Particle<DIMS>) -> [f64; DIMS] {}
impl<F, const DIMS: usize> VelocityFunction<DIMS> for F where
    F: Fn(&Particle<DIMS>, &Particle<DIMS>) -> [f64; DIMS]
{
}

pub trait ObjectiveFunction<const DIMS: usize>: Fn(&[f64; DIMS]) -> f64 {}
impl<F, const DIMS: usize> ObjectiveFunction<DIMS> for F where F: Fn(&[f64; DIMS]) -> f64 {}

pub fn velocity(current: &Particle<2>, best: &Particle<2>) -> [f64; 2] {
    let c1 = 1.0;
    let c2 = 1.0;
    let w = 0.5;
    let (r1, r2): (f64, f64) = rand::random();

    let x = w * current.velocity()[0]
        + c1 * r1 * (best.coordinates()[0] - current.coordinates()[0])
        + c2 * r2 * (current.best()[0] - current.coordinates()[0]);
    let y = w * current.velocity()[1]
        + c1 * r1 * (best.coordinates()[1] - current.coordinates()[1])
        + c2 * r2 * (current.best()[1] - current.coordinates()[1]);

    [x, y]
}
