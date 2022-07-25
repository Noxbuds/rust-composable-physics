use std::sync::mpsc::{Sender, Receiver};

use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

use crate::{particle::{Particle}, components::PhysicsComponent};
use crate::utils::Vec2;

pub struct App {
    pub gl: GlGraphics,
    pub particles: Vec<Particle>,
    pub particle_channel: (Sender<Particle>, Receiver<Particle>),
    pub world_scale: f64,
    pub sub_steps: i32,
    pub components: Vec<Box<dyn PhysicsComponent>>,
}

fn world_to_screen(world_scale: f64, world: Vec2) -> Vec2 {
    Vec2 {
        x: world_scale * world.x,
        y: world_scale * world.y,
    }
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for particle in &self.particles {
                let position = world_to_screen(self.world_scale, particle.position);
                
                let rect = rectangle::square(position.x, position.y, particle.radius * self.world_scale);
                ellipse(particle.color, rect, c.transform, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let particles_clone = self.particles.clone();
        let (sender, receiver) = &self.particle_channel;

        let dt = args.dt / self.sub_steps as f64;
        for _ in 0..self.sub_steps {
            for (i, particle) in self.particles.iter_mut().enumerate() {
                for component in &self.components {
                    if component.allow(particle) {
                        component.apply(particle, i, &particles_clone, dt);
                    }
                }

                particle.update_position(dt);
            }
        }

        for component in &mut self.components {
            component.update_system(args.dt, sender);
        }

        for particle in receiver.try_iter() {
            self.particles.push(particle);
        }
    }
}
