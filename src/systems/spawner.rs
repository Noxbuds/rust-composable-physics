use crate::{utils::Vec2, entity::Entity, components::{position::Position, Component, acceleration::Acceleration, circle_collider::CircleCollider}};
use super::System;

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

impl System for Spawner {
    fn apply(&mut self, entities: &mut crate::entity::EntityContainer, dt: f64) {
        self.timer += dt;

        if self.timer >= self.spawn_time && self.count > 0 {
            self.timer = 0.0;
            self.count -= 1;

            let position = (self.get_spawn_point)(dt);
            let radius = (self.get_radius)(dt);
            let velocity = (self.get_velocity)(dt);
            // let color = (self.get_color)(dt);

            // TODO: better entity IDs
            let mut entity = Entity::new(self.count);
            Position::attach(&mut entity, Position::from_vec2(position - velocity * dt), 1);
            Position::attach(&mut entity, Position::from_vec2(position), 0);
            Acceleration::attach(&mut entity, Acceleration { x: 0.0, y: 0.0 }, 0);
            CircleCollider::attach(&mut entity, CircleCollider { radius }, 0);

            entities.insert(entity.id, entity);
        }
    }
}
