use crate::{entity::EntityContainer, components::{position::Position, Component, acceleration::Acceleration}};

use super::System;

pub struct VerletIntegrator;

impl System for VerletIntegrator {
    fn apply(&mut self, entities: &mut EntityContainer, dt: f64) {
        for entity in entities.values_mut() {
            let maybe_position = Position::get(entity, 0);
            let maybe_old_position = Position::get(entity, 1);
            let maybe_acceleration = Acceleration::get(entity, 0);

            if let (Some(position), Some(old_position), Some(acceleration)) = (maybe_position, maybe_old_position, maybe_acceleration) {
                // verlet integration
                let x = position.x * 2.0 - old_position.x + acceleration.x * dt * dt;
                let y = position.y * 2.0 - old_position.y + acceleration.y * dt * dt;

                let new_pos = Position { x, y };
                Position::attach(entity, new_pos, 0);
                Position::attach(entity, position, 1);
                Acceleration::attach(entity, Acceleration { x: 0.0, y: 0.0 }, 0);
            }
        }
    }
}
