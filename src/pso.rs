use crate::{
    bounds::Bound, particle::Particle, vector::Vector, ObjectiveFunction, VelocityFunction,
};

// Runs the PSO algorithm
// The velocity function as well as the objective function should be specified.
//
// A vector containing the best coordinates of the last generation is returned.
// # Example
//
// ```
// // the canonical velocity equation
// let velocity = |current, best| {
//     let c1 = 1.0;
//     let c2 = 1.0;
//     let w = 0.5;
//
//     // from the rand crate
//     let (r1, r2): (f64, f64) = rand::random();
//     let vel = w * current.velocity()
//        + c1 * r1 * (best.coordinates() - current.coordinates())
//        + c2 * r2 * (current.best() - current.coordinates());
//
//     vel
// }
//
// let objective = |particle| {
//
// }
//
// let coords = pso(100, 50, &[Bound::from(-10.0, 10.0); 2], velocity, )
// println!("{:?}", coords);
// ```
pub fn pso<V, F, const DIMS: usize>(
    pop_size: usize,
    generations: usize,
    bounds: &[Bound],
    velocity: V,
    objective: F,
) -> Vector<DIMS>
where
    V: VelocityFunction<DIMS>,
    F: ObjectiveFunction<DIMS>,
{
    assert_eq!(bounds.len(), DIMS);
    // creating random particles
    let mut particles = create_particles(pop_size, bounds);

    // adding best position of every particle
    particles.iter_mut().for_each(|p| {
        p.apply_function(&objective);
    });

    // getting global best
    let mut g_best = particles
        .iter()
        .min_by(|p1, p2| objective(&p1.coordinates()).total_cmp(&objective(&p2.coordinates())))
        .unwrap()
        .clone(); // sadly the clone is necessary

    for _ in 0..generations {
        for p in &mut particles {
            p.apply_velocity(&g_best, &velocity);
        }

        g_best = particles
            .iter()
            .min_by(|p1, p2| objective(&p1.coordinates()).total_cmp(&objective(&p2.coordinates())))
            .unwrap()
            .clone(); // sadly the clone is necessary
    }

    g_best.coordinates()
}

fn create_particles<const DIMS: usize>(pop_size: usize, bounds: &[Bound]) -> Vec<Particle<DIMS>> {
    // creating random particles
    (0..pop_size)
        .map(|_| Particle::new_random(&bounds))
        .collect()
}
