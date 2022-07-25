use std::sync::mpsc::Sender;
use crate::{particle::Particle, utils::Vec2};
use super::PhysicsComponent;

type GetSpawnPoint = Box<dyn Fn(f64) -> Vec2>;
type GetRadius = Box<dyn Fn(f64) -> f64>;

pub struct Spawner {
    pub spawn_time: f64,
    pub get_spawn_point: GetSpawnPoint,
    pub get_radius: GetRadius,
    count: i32,
    timer: f64
}

impl Spawner {
    pub fn new(count: i32, spawn_time: f64, get_spawn_point: GetSpawnPoint, get_radius: GetRadius) -> Spawner {
        Spawner { count, timer: 0.0, spawn_time, get_spawn_point, get_radius }
    }
}

impl PhysicsComponent for Spawner {
    fn update_system(&mut self, dt: f64, particle_channel: &Sender<Particle>) {
        self.timer += dt;

        if self.timer >= self.spawn_time && self.count > 0 {
            self.timer = 0.0;
            self.count -= 1;

            let position = (self.get_spawn_point)(dt);
            let radius = (self.get_radius)(dt);

            let new_particle = Particle {
                old_position: position,
                position,
                acceleration: Vec2 { x: 0.0, y: 0.0 },
                radius,
                color: [1.0, 1.0, 1.0, 1.0],
            };

            particle_channel.send(new_particle).unwrap();
        }
    }
}
