use app::App;
use opengl_graphics::{GlGraphics, OpenGL};
use simulation::Simulation;
use utils::Vec2;
use rand::random;
use piston::{WindowSettings, EventSettings, Events, RenderEvent, UpdateEvent};
use glutin_window::GlutinWindow;
use components::{gravity::Gravity, circle_walls::CircleWalls, spawner::Spawner, collision::{Collision, Quad}, integrator::VerletIntegrator};

mod app;
mod particle;
mod components;
mod utils;
mod simulation;

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
    
    let simulation = Simulation::new(world_scale, 8)
        .add_component(Box::new(Gravity {
            strength: Vec2 { x: 0.0, y: 160.0 }
        }))
        .add_component(Box::new(CircleWalls {
            center: Vec2 { x: window_width as f64, y: window_height as f64 } * 0.5 / world_scale,
            radius: 400.0,
        }))
        .add_component(Box::new(Spawner {
            timer: 0.0,
            count: 5000,
            spawn_time: 0.03,
            get_spawn_point: Box::new(move |_| {
                let center = Vec2 { x: window_width as f64, y: window_height as f64 } * 0.5 / world_scale;
                center + Vec2 { x: 20.0, y: -200.0 } 
            }),
            get_radius: Box::new(move |_| {
                let roll = random::<f64>();
                2.0 + roll * 8.0
            }),
            get_velocity: Box::new(move |_| {
                Vec2 {
                    x: (random::<f64>() * 2.0 - 1.0) * 100.0 + 200.0,
                    y: 1500.0,
                }
            }),
            get_color: Box::new(move |_| {
                [random(), random(), random(), 1.0]
            }),
        }))
        .add_component(Box::new(Collision {
            tree: Quad {
                dimensions: [0.0, 0.0, window_width as f64 / world_scale, window_height as f64 / world_scale],
                children: None,
                particle_ids: vec![],
                max_particles: 10
            }
        }))
        .add_component(Box::new(VerletIntegrator {}));

    let mut app = App {
        gl: GlGraphics::new(opengl),
        simulation,
        render_dt: 0.0,
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
