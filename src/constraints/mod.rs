use crate::particle::{Particle};

pub mod gravity;

pub trait PhysicsConstraint {
    fn allow(&self, particle: &Particle) -> bool {
        true
    }
    fn apply(&self, particle: &mut Particle, all_particles: &[Particle]);
}
