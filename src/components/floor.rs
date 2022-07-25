use crate::particle::Particle;
use super::PhysicsComponent;

pub struct Floor {
    pub y: f64,
}

impl PhysicsComponent for Floor {
    fn apply(&self, particle: &mut Particle, _all_particles: &[Particle], _: f64) {
        let edge_y = particle.position.y + particle.radius;

        if edge_y > self.y {
            particle.position.y -= edge_y - self.y;
        }
    }
}
