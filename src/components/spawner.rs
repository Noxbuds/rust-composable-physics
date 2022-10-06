use std::sync::mpsc::Sender;
use crate::{particle::Particle, utils::Vec2};
use super::PhysicsComponent;

type GetVector = Box<dyn Fn(f64) -> Vec2>;
type GetRadius = Box<dyn Fn(f64) -> f64>;
type GetColor = Box<dyn Fn(f64) -> [f32; 4]>;

pub struct Spawner {
    pub spawn_time: f64,
    pub get_spawn_point: GetVector,
    pub get_radius: GetRadius,
    pub get_velocity: GetVector,
    pub get_color: GetColor,
    pub count: i32,
    pub timer: f64
}

impl PhysicsComponent for Spawner {
    fn update_system(&mut self, dt: f64, particle_channel: &Sender<Particle>, _particles: &Vec<Particle>) {
        self.timer += dt;

        if self.timer >= self.spawn_time && self.count > 0 {
            self.timer = 0.0;
            self.count -= 1;

            let position = (self.get_spawn_point)(dt);
            let radius = (self.get_radius)(dt);
            let velocity = (self.get_velocity)(dt);
            let color = (self.get_color)(dt);

            let new_particle = Particle {
                old_position: position - velocity * dt,
                position,
                acceleration: Vec2 { x: 0.0, y: 0.0 },
                radius,
                color,
            };

            particle_channel.send(new_particle).unwrap();
        }
    }
}
