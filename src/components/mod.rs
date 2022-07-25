use std::sync::mpsc::Sender;

use crate::particle::{Particle};

pub mod gravity;
pub mod floor;
pub mod circle_walls;
pub mod spawner;
pub mod collision;

pub trait PhysicsComponent {
    fn allow(&self, _particle: &Particle) -> bool {
        true
    }

    fn apply(&self, _particle: &mut Particle, _id: usize, _all_particles: &[Particle], _dt: f64) {}

    fn update_system(&mut self, _dt: f64, _particle_channel: &Sender<Particle>) {}
}
