use crate::utils::Vec2;

use super::PhysicsComponent;

pub struct VerletIntegrator {

}

impl PhysicsComponent for VerletIntegrator {
    fn apply(&self, particles: &mut Vec<crate::particle::Particle>, dt: f64) {
        for particle in particles {
            // verlet integration
            let new_position = particle.position * 2.0 - particle.old_position + particle.acceleration * dt * dt;

            particle.old_position = particle.position;
            particle.position = new_position;
            particle.acceleration = Vec2 { x: 0.0, y: 0.0 };
        }
    }
}
