use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector<const DIMS: usize>([f64; DIMS]);

impl<const DIMS: usize> Vector<DIMS> {
    pub fn new(coords: [f64; DIMS]) -> Self {
        Self(coords)
    }

    pub fn size(&self) -> usize {
        DIMS
    }
}

impl<const DIMS: usize> ops::Index<usize> for Vector<DIMS> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const DIMS: usize> ops::Add<Vector<DIMS>> for Vector<DIMS> {
    type Output = Self;

    fn add(mut self, rhs: Vector<DIMS>) -> Self::Output {
        for (l, r) in self.0.iter_mut().zip(rhs.0) {
            *l += r;
        }
        self
    }
}

impl<const DIMS: usize> ops::AddAssign<Vector<DIMS>> for Vector<DIMS> {
    fn add_assign(&mut self, rhs: Vector<DIMS>) {
        for (l, r) in self.0.iter_mut().zip(rhs.0) {
            *l += r;
        }
    }
}

impl<const DIMS: usize> ops::Sub<Vector<DIMS>> for Vector<DIMS> {
    type Output = Self;

    fn sub(mut self, rhs: Vector<DIMS>) -> Self::Output {
        for (l, r) in self.0.iter_mut().zip(rhs.0) {
            *l -= r;
        }
        self
    }
}

impl<const DIMS: usize> ops::SubAssign<Vector<DIMS>> for Vector<DIMS> {
    fn sub_assign(&mut self, rhs: Vector<DIMS>) {
        for (l, r) in self.0.iter_mut().zip(rhs.0) {
            *l -= r;
        }
    }
}

impl<const DIMS: usize> ops::Mul<f64> for Vector<DIMS> {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        for l in self.0.iter_mut() {
            *l *= rhs;
        }
        self
    }
}

impl<const DIMS: usize> ops::MulAssign<Vector<DIMS>> for Vector<DIMS> {
    fn mul_assign(&mut self, rhs: Vector<DIMS>) {
        for (l, r) in self.0.iter_mut().zip(rhs.0) {
            *l *= r;
        }
    }
}

// implementation for 64
impl<const DIMS: usize> ops::Mul<Vector<DIMS>> for f64 {
    type Output = Vector<DIMS>;

    fn mul(self, rhs: Vector<DIMS>) -> Self::Output {
        rhs * self
    }
}
