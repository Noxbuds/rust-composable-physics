use crate::particle::Particle;
use super::PhysicsComponent;

pub struct Collision {

}

impl PhysicsComponent for Collision {
    fn apply(&self, particles: &mut Vec<Particle>, _dt: f64) {
        for i in 0..particles.len() {
            for j in i + 1..particles.len() {
                let dir = particles[i].position - particles[j].position;
                let radius_sum = particles[i].radius + particles[j].radius;
                let sqr_dist = dir.sqr_len();

                if sqr_dist < radius_sum * radius_sum {
                    let dist = sqr_dist.sqrt();
                    let axis = dir / dist;
                    let delta = radius_sum - dist;

                    particles[i].position += axis * 0.5 * delta;
                    particles[j].position -= axis * 0.5 * delta;
                }
            }
        }
    }
}
