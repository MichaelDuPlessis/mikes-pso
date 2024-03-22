use super::Particle;
use crate::allocator::Size;
use rand::distributions::{Distribution, Uniform};
use std::ops::{self, AddAssign};

/// This is just a 3 dimensional vecto
struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> ops::Add for Vec3<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> ops::AddAssign for Vec3<T>
where
    T: ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

/// This is a particle in PSO
pub struct VecParticle<T> {
    coordinates: Vec3<T>,
    velocity: Vec3<T>,
}

/*
impl Particle<f32> for VecParticle<f32> {
    fn new(dims: impl Size) -> Self {
        let mut rng = rand::thread_rng();
        let between = Uniform::new_inclusive(f32::MIN, f32::MAX);

        let particles = (0..dims.size()).map(|_| between.sample(&mut rng)).collect();

        Self(particles)
    }

    fn coord(&self) -> &f32 {
        todo!()
    }

    fn coord_mut(&self) -> &mut f32 {
        todo!()
    }

    fn vec(&self) -> &f32 {
        todo!()
    }

    fn vec_mut(&self) -> &mut f32 {
        todo!()
    }
}
*/
