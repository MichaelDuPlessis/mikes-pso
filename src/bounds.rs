// represents is the bounds for where the particles can be spawned

#[derive(Clone, Copy)]
pub struct Bound(pub (f64, f64));

impl From<(f64, f64)> for Bound {
    fn from(bound: (f64, f64)) -> Self {
        Self(bound)
    }
}
