use std::sync::mpsc::Sender;

use crate::particle::{Particle};

pub mod gravity;
pub mod floor;
pub mod circle_walls;
pub mod spawner;
pub mod collision;
pub mod integrator;
pub mod position;

pub trait PhysicsComponent {
    fn allow(&self, _particle: &Particle) -> bool {
        true
    }

    fn apply(&self, _particles: &mut Vec<Particle>, _dt: f64) {}

    fn update_system(&mut self, _dt: f64, _particle_channel: &Sender<Particle>) {}
}
