pub mod bounds;
mod particle;
pub mod pso;
pub mod vector;

use particle::Particle;
use vector::Vector;

// using it as a trait bound alias
// particle should contain all the needed information
/// This trait is used as a marker to reprsent a valid velocity equation
pub trait VelocityFunction<const DIMS: usize>: Fn(/*current*/ &Particle<DIMS>, /*best*/ &Particle<DIMS>) -> Vector<DIMS> {}
impl<F, const DIMS: usize> VelocityFunction<DIMS> for F where
    F: Fn(&Particle<DIMS>, &Particle<DIMS>) -> Vector<DIMS>
{
}

// using it as a trait bound alias
/// This trait is used as a marker to reprsent a valid objective equation
pub trait ObjectiveFunction<const DIMS: usize>: Fn(&Vector<DIMS>) -> f64 {}
impl<F, const DIMS: usize> ObjectiveFunction<DIMS> for F where F: Fn(&Vector<DIMS>) -> f64 {}

/// This is the canonical velocity equation and can be used if a custom one is not needed
pub fn canonical_velocity<const DIMS: usize>(
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

#[cfg(test)]
mod tests {
    use crate::{bounds::Bound, pso::pso};

    use super::*;

    pub struct Function<const SIZE: usize> {
        pub func: Box<dyn Fn(&Vector<SIZE>) -> f64 + Send>,
        pub minima: f64,
        pub bounds: Vec<Bound>,
    }

    #[test]
    fn it_works() {
        let f = |current: &Particle<2>, best: &Particle<2>| {
            (current.coordinates() - best.coordinates()) - current.coordinates()
        };
        let func = Function {
            func: Box::new(|coords: &Vector<2>| {
                4.0 * coords[0] * coords[0] - 2.1 * coords[0] * coords[0] * coords[0] * coords[0]
                    + (coords[0] * coords[0] * coords[0] * coords[0] * coords[0] * coords[0]) / 3.0
                    + coords[0] * coords[1]
                    - 4.0 * coords[1] * coords[1]
                    + 4.0 * coords[1] * coords[1] * coords[1] * coords[1]
            }),
            minima: -1.0316,
            bounds: vec![Bound::from((-5.0, 5.0)); 2],
        };

        let particle = pso(100, 1, &func.bounds, &f, &func.func);
        println!("{particle:?}");
    }
}
