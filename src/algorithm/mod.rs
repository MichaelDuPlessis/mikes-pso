/// The algorithm to use for PSO
pub trait Algorithm<P, I>
where
    P: Particle<I>,
{
    fn run(particles: &[P]);
}
