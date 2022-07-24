use app::App;
use opengl_graphics::{GlGraphics, OpenGL};
use particle::{Particle, Vec2};
use piston::{WindowSettings, EventSettings, Events, RenderEvent, UpdateEvent};
use glutin_window::GlutinWindow;

mod app;
mod particle;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Physics simulator", [1920, 1080])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let scale: f64 = 4.0;
    let test_particle = Particle {
        old_position: Vec2 { x: 960.0 / scale, y: 540.0 / scale },
        position: Vec2 { x: 960.0 / scale, y: 540.0 / scale },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        radius: 10.0,
        color: [1.0, 1.0, 1.0, 1.0],
    };
    let particles = vec![test_particle];

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        particles,
        world_scale: scale,
    };

    
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
