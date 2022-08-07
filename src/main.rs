use std::{sync::mpsc, collections::HashMap};
use app::App;
use component::Component;
use entity::Entity;
use opengl_graphics::{GlGraphics, OpenGL};
use utils::Vec2;
use rand::random;
use piston::{WindowSettings, EventSettings, Events, RenderEvent, UpdateEvent};
use glutin_window::GlutinWindow;
use components::{gravity::Gravity, circle_walls::CircleWalls, spawner::Spawner, collision::Collision, integrator::VerletIntegrator, position::Position};

mod app;
mod particle;
mod entity;
mod components;
mod component;
mod system;
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

    let mut test_entity = Entity::new(0);
    let test_position = Position { x: 1.0, y: 2.0 };
    Position::attach(&mut test_entity, test_position);

    let mut entities: HashMap<i32, Entity> = HashMap::new();
    entities.insert(test_entity.id, test_entity);
    
    let mut app = App {
        gl: GlGraphics::new(opengl),
        particles: Vec::new(),
        particle_channel: mpsc::channel(),
        entities,
        world_scale,
        sub_steps: 8,
        components: vec![
            Box::new(Gravity {
                strength: Vec2 { x: 0.0, y: 160.0 }
            }), 
            Box::new(CircleWalls {
                center: Vec2 {
                    x: window_width as f64 * 0.5 / world_scale,
                    y: window_height as f64 * 0.5 / world_scale
                },
                radius: 400.0
            }),
            Box::new(Spawner {
                timer: 0.0,
                count: 10000,
                spawn_time: 0.03,
                get_spawn_point: Box::new(move |_| {
                    Vec2 {
                        x: window_width as f64 * 0.5 / world_scale + 20.0,
                        y: window_height as f64 * 0.5 / world_scale - 200.0
                    }
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
            }),
            Box::new(Collision {}),
            Box::new(VerletIntegrator {}),
        ],
        physics_dt: 0.0,
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
