use crate::particle::{Particle};
use crate::utils::Vec2;

use super::PhysicsComponent;

pub struct CircleWalls {
    pub radius: f64,
    pub center: Vec2,
}

impl PhysicsComponent for CircleWalls {
    fn apply(&self, particles: &mut Vec<Particle>, _dt: f64) {
        for particle in particles {
            let dir = particle.position - self.center;
            let dir_len = dir.len();
            
            if dir_len > self.radius - particle.radius {
                let axis = dir / dir_len;
                particle.position -= axis * (dir_len + particle.radius - self.radius);
            }
        }
    }
}
