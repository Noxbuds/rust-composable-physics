use crate::components::Component;
use crate::components::circle_collider::CircleCollider;
use crate::components::position::Position;
use crate::entity::EntityContainer;
use crate::utils::Vec2;

use super::System;

pub struct CircleWalls {
    pub radius: f64,
    pub center: Vec2,
}

impl System for CircleWalls {
    fn apply(&mut self, entities: &mut EntityContainer, _dt: f64) {
        for entity in entities.values_mut() {
            let maybe_position = Position::get(entity, 0);
            let maybe_collider = CircleCollider::get(entity, 0);

            if let (Some(position), Some(collider)) = (maybe_position, maybe_collider) {
                let position_vec = position.to_vec2();
                let dir = position_vec - self.center;
                let dir_len = dir.len();

                if dir_len > self.radius - collider.radius {
                    let axis = dir / dir_len;
                    let new_position = position_vec - axis * (dir_len + collider.radius - self.radius);

                    Position::attach(entity, Position::from_vec2(new_position), 0);
                }
            }
        }
    }
}
