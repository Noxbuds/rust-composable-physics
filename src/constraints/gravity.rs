use crate::particle::{Vec2, Particle};

use super::PhysicsConstraint;

pub struct Gravity;

impl PhysicsConstraint for Gravity {
    fn apply(&self, particle: &mut Particle, _: &[Particle]) {
        let acc = Vec2 { x: 0.0, y: 20.0 };
        particle.add_acceleration(acc);
    }
}
