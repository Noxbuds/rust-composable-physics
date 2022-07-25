use crate::particle::{Particle};

pub mod gravity;
pub mod floor;
pub mod circle_walls;

pub trait PhysicsComponent {
    fn allow(&self, _particle: &Particle) -> bool {
        true
    }
    fn apply(&self, particle: &mut Particle, all_particles: &[Particle]);
}
