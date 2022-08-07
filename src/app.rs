use std::{sync::mpsc::{Sender, Receiver}, collections::HashMap};

use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use piston::{RenderArgs, UpdateArgs};

use crate::{particle::{Particle}, components::PhysicsComponent, entity::Entity};
use crate::utils::Vec2;

pub struct App {
    pub gl: GlGraphics,
    pub particles: Vec<Particle>,
    pub particle_channel: (Sender<Particle>, Receiver<Particle>),
    pub entities: HashMap<i32, Entity>,
    pub world_scale: f64,
    pub sub_steps: i32,
    pub components: Vec<Box<dyn PhysicsComponent>>,
    pub physics_dt: f64,
    pub render_dt: f64,
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

        let timestep_text = format!("timestep: {:.1} ms", self.physics_dt * 1000.0);
        let frame_time_text = format!("frame time: {:.1} ms", self.render_dt * 1000.0);
        let particle_count = format!("particle count: {:.1}", self.particles.len());
        let stats = vec![timestep_text, frame_time_text, particle_count];

        let font_size = 32;
        let text_gap: f64 = 12.0;

        let mut glyph_cache = GlyphCache::new("assets/Roboto-Regular.ttf", (), TextureSettings::new()).unwrap();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for i in 0..stats.len() {
                let n = i as f64 + 1.0;
                text([1.0, 1.0, 1.0, 1.0], font_size, &stats.get(i).unwrap(), &mut glyph_cache, c.transform.trans(12.0, n * font_size as f64 + n * text_gap), gl).unwrap();
            }
        });
        
        for particle in &self.particles {
            self.gl.draw(args.viewport(), |c, gl| {
                let position = world_to_screen(self.world_scale, particle.position);
                
                let rect = rectangle::square(position.x - particle.radius, position.y - particle.radius, particle.radius * 2.0 * self.world_scale);
                ellipse(particle.color, rect, c.transform, gl);
            });
        }

        self.render_dt = args.ext_dt;
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let (sender, receiver) = &self.particle_channel;

        let dt = args.dt / self.sub_steps as f64;
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

        self.physics_dt = args.dt;
    }
}
