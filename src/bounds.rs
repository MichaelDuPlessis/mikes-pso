// represents is the bounds for where the particles can be spawned
/// This is used to bound a particle into a certain range when it is first created.
#[derive(Clone, Copy)]
pub struct Bound(pub (f64, f64));

impl From<(f64, f64)> for Bound {
    fn from(bound: (f64, f64)) -> Self {
        Self(bound)
    }
}
