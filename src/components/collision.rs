use crate::particle::Particle;
use super::PhysicsComponent;

#[derive(Debug)]
pub struct Quad {
    pub dimensions: [f64; 4],
    pub children: Option<[Box<Quad>; 4]>,
    pub particle_ids: Vec<usize>,
    pub max_particles: usize,
}

impl Quad {
    fn divide(&mut self) {
        let positions = [
            [0.0, 0.0],
            [0.0, 0.5],
            [0.5, 0.0],
            [0.5, 0.5],
        ];

        let children = positions.map(|position| {
            let child = Box::new(Quad {
                dimensions: [
                    self.dimensions[0] + position[0] * self.dimensions[2],
                    self.dimensions[1] + position[1] * self.dimensions[3],
                    self.dimensions[2] * 0.5,
                    self.dimensions[3] * 0.5,
                ],
                children: None,
                particle_ids: vec![],
                max_particles: self.max_particles,
            });

            dbg!(&child);

            child
        });

        self.children = Some(children);
    }

    pub fn balance(&mut self, particles: &Vec<Particle>) {
        let bounded_particles: Vec<&Particle> = particles.iter()
            .filter_map(|particle| {
                let x_bounds = particle.position.x >= self.dimensions[0] && particle.position.x < self.dimensions[0] + self.dimensions[2];
                let y_bounds = particle.position.y >= self.dimensions[1] && particle.position.y < self.dimensions[1] + self.dimensions[3];

                if x_bounds && y_bounds {
                    Some(particle)
                } else {
                    None
                }
            })
            .collect();

        if bounded_particles.len() <= self.max_particles {
            if let None = self.children {
                self.particle_ids = bounded_particles.iter().enumerate().map(|(i, _)| i).collect();
            }
        } else if let None = self.children {
            self.divide();
            println!("dividing");
        }

        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                child.balance(particles);
            }
        }
    }

    fn handle_collisions(&self, particles: &mut Vec<Particle>) {
        for i in 0..self.particle_ids.len() {
            for j in i + 1..self.particle_ids.len() {
                let id_i = self.particle_ids[i];
                let id_j = self.particle_ids[j];

                let dir = particles[id_i].position - particles[id_j].position;
                let radius_sum = particles[id_i].radius + particles[id_j].radius;
                let sqr_dist = dir.sqr_len();

                if sqr_dist < radius_sum * radius_sum {
                    let dist = sqr_dist.sqrt();
                    let axis = dir / dist;
                    let delta = radius_sum - dist;

                    particles[id_i].position += axis * 0.5 * delta;
                    particles[id_j].position -= axis * 0.5 * delta;
                }
            }
        }
    }

    pub fn apply(&self, particles: &mut Vec<Particle>) {
        self.handle_collisions(particles);
        if let Some(children) = &self.children {
            for child in children {
                child.handle_collisions(particles);
            }
        }
    }
}

pub struct Collision {
    pub tree: Quad,
}

impl PhysicsComponent for Collision {
    fn apply(&self, particles: &mut Vec<Particle>, _dt: f64) {
        self.tree.apply(particles);
    }

    fn update_system(&mut self, _dt: f64, _particle_channel: &std::sync::mpsc::Sender<Particle>, particles: &Vec<Particle>) {
        self.tree.balance(particles);
    }
}
