use crate::particle::particle::Particle;

/// The algorithm to use for PSO
pub trait Algorithm<P>
where
    P: Particle,
{
    fn search<'a>(&'a self, particles: impl IntoIterator<Item = &'a mut P>)
    where
        P: 'a;
}

/// The canconical implementaion of the PSO algorithm
pub struct Canonical;

impl<P> Algorithm<P> for Canonical
where
    P: Particle,
{
    fn search<'a>(&'a self, particles: impl IntoIterator<Item = &'a mut P>)
    where
        P: 'a,
    {
        let particles = particles.into_iter();
    }
}
