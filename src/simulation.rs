use std::sync::mpsc::{Sender, Receiver, self};

use crate::{particle::Particle, components::PhysicsComponent};

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub particle_channel: (Sender<Particle>, Receiver<Particle>),
    pub world_scale: f64,
    pub sub_steps: i32,
    pub components: Vec<Box<dyn PhysicsComponent>>,
}

impl Simulation {
    pub fn new(world_scale: f64, sub_steps: i32) -> Simulation {
        Simulation {
            particles: vec![],
            particle_channel: mpsc::channel(),
            world_scale,
            sub_steps,
            components: vec![]
        }
    }

    pub fn add_component(mut self, component: Box<dyn PhysicsComponent>) -> Self {
        self.components.push(component);
        return self;
    }

    pub fn update(&mut self, timestep: f64) {
        let (sender, receiver) = &self.particle_channel;

        let dt = timestep / self.sub_steps as f64;
        for _ in 0..self.sub_steps {
            for component in &self.components {
                component.apply(&mut self.particles, dt);
            }

            for component in &mut self.components {
                component.update_system(dt, sender);
            }
        }

        for particle in receiver.try_iter() {
            self.particles.push(particle);
        }
    }
}
