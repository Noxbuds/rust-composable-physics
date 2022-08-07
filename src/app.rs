use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use piston::{RenderArgs, UpdateArgs};

use crate::simulation::Simulation;
use crate::utils::Vec2;

pub struct App {
    pub gl: GlGraphics,
    pub simulation: Simulation,
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

        let frame_time_text = format!("frame time: {:.1} ms", self.render_dt * 1000.0);
        let particle_count = format!("particle count: {:.1}", self.simulation.particles.len());
        let stats = vec![frame_time_text, particle_count];

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
        
        for particle in &self.simulation.particles {
            self.gl.draw(args.viewport(), |c, gl| {
                let position = world_to_screen(self.simulation.world_scale, particle.position);
                
                let rect = rectangle::square(position.x - particle.radius, position.y - particle.radius, particle.radius * 2.0 * self.simulation.world_scale);
                ellipse(particle.color, rect, c.transform, gl);
            });
        }

        self.render_dt = args.ext_dt;
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.simulation.update(args.dt);
    }
}
