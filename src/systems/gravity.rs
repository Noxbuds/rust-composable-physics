use crate::components::acceleration::Acceleration;
use crate::entity::EntityContainer;
use crate::utils::Vec2;

use super::System;

pub struct Gravity {
    pub strength: Vec2,
}

impl System for Gravity {
    fn apply(&mut self, entities: &mut EntityContainer, _dt: f64) {
        for entity in entities.values_mut() {
            Acceleration::accelerate(entity, self.strength);
        }
    }
}
