use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

use crate::{particle::{Particle, Vec2}, constraints::PhysicsConstraint};

pub struct App {
    pub gl: GlGraphics,
    pub rotation: f64,
    pub particles: Vec<Particle>,
    pub world_scale: f64,
    pub constraints: Vec<Box<dyn PhysicsConstraint>>,
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

                // let transform = c.transform.trans(position.x, position.y);
                
                let rect = rectangle::square(position.x, position.y, particle.radius * self.world_scale);
                ellipse(particle.color, rect, c.transform, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let particles_clone = self.particles.clone();

        for particle in &mut self.particles {
            for constraint in &self.constraints {
                if constraint.allow(particle) {
                    constraint.apply(particle, &particles_clone);
                }

                particle.update_position(args.dt);
            }
        }
    }
}
