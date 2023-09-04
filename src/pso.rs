use crate::{bounds::Bound, particle::Particle, ObjectiveFunction, VelocityFunction};

pub fn pso<V, F, const DIMS: usize>(
    pop_size: usize,
    generations: usize,
    bounds: &[Bound],
    velocity: V,
    objective: F,
) -> Particle<DIMS>
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

    g_best
}

fn create_particles<const DIMS: usize>(pop_size: usize, bounds: &[Bound]) -> Vec<Particle<DIMS>> {
    // creating random particles
    (0..pop_size)
        .map(|_| Particle::new_random(&bounds))
        .collect()
}
