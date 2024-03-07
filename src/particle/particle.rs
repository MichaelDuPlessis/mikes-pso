use std::ops;

/// This is a particle in PSO
/// In essence it is just a which makes up the "coordinates" of the particle
pub struct Particle(Vec<f32>);

impl ops::Add for Particle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // getting values inside Particle
        let Self(mut lhs) = self;
        let Self(rhs) = rhs;

        // adding rhs to lhs
        for (l, r) in lhs.iter_mut().zip(rhs) {
            *l += r;
        }

        Self(lhs)
    }
}

impl ops::AddAssign for Particle {
    fn add_assign(&mut self, rhs: Self) {
        let Self(lhs) = self;
        let Self(rhs) = rhs;

        for (l, r) in lhs.iter_mut().zip(rhs) {
            *l += r;
        }
    }
}
