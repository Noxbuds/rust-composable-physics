use crate::particle::Particle;
use super::PhysicsComponent;

pub struct Collision {

}

impl PhysicsComponent for Collision {
    fn apply(&self, particle: &mut Particle, id: usize, all_particles: &[Particle], _dt: f64) {
        for i in id + 1..all_particles.len() {
            let other = all_particles[i];
            let dir = particle.position - other.position;
            let dist = dir.len();

            if dist < particle.radius + other.radius {
                let axis = dir / dist;
                let delta = particle.radius + other.radius - dist;

                particle.position += axis * 0.5 * delta;
            }
        }
    }
}
