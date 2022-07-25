use crate::particle::{Particle};
use crate::utils::Vec2;

use super::PhysicsComponent;

pub struct Gravity {
    pub strength: Vec2,
}

impl PhysicsComponent for Gravity {
    fn apply(&self, particle: &mut Particle, _: &[Particle], _: f64) {
        let acc = Vec2 { x: self.strength.x, y: self.strength.y };
        particle.add_acceleration(acc);
    }
}
