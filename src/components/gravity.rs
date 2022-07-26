use crate::particle::{Particle};
use crate::utils::Vec2;

use super::PhysicsComponent;

pub struct Gravity {
    pub strength: Vec2,
}

impl PhysicsComponent for Gravity {
    fn apply(&self, particles: &mut Vec<Particle>, _dt: f64) {
        let acc = Vec2 { x: self.strength.x, y: self.strength.y };

        for particle in particles {
            particle.add_acceleration(acc);
        }
    }
}
