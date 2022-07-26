use std::sync::mpsc;
use app::App;
use opengl_graphics::{GlGraphics, OpenGL};
use utils::Vec2;
use piston::{WindowSettings, EventSettings, Events, RenderEvent, UpdateEvent};
use glutin_window::GlutinWindow;
use components::{gravity::Gravity, circle_walls::CircleWalls, spawner::Spawner, collision::Collision, integrator::VerletIntegrator};

mod app;
mod particle;
mod components;
mod utils;

fn main() {
    let opengl = OpenGL::V3_2;

    let window_width = 1920;
    let window_height = 1080;
    let world_scale: f64 = 1.0;

    let mut window: GlutinWindow = WindowSettings::new("Physics simulator", [window_width, window_height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        particles: Vec::new(),
        particle_channel: mpsc::channel(),
        world_scale,
        sub_steps: 16,
        components: vec![
            Box::new(Gravity {
                strength: Vec2 { x: 0.0, y: 160.0 }
            }), 
            Box::new(CircleWalls {
                center: Vec2 {
                    x: window_width as f64 * 0.5 / world_scale,
                    y: window_height as f64 * 0.5 / world_scale
                },
                radius: 320.0
            }),
            Box::new(Spawner {
                timer: 0.0,
                count: 2500,
                spawn_time: 0.02,
                get_spawn_point: Box::new(move |_| {
                    Vec2 {
                        x: window_width as f64 * 0.5 / world_scale + 20.0,
                        y: window_height as f64 * 0.5 / world_scale
                    }
                }),
                get_radius: Box::new(|_| {
                    2.0
                }),
                get_velocity: Box::new(move |_| {
                    Vec2 {
                        x: 0.0,
                        y: 1000.0,
                    }
                }),
            }),
            Box::new(Collision {}),
            Box::new(VerletIntegrator {}),
        ],
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
