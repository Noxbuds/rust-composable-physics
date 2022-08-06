pub mod gravity;
pub mod circle_walls;
pub mod spawner;
pub mod collision;
pub mod integrator;

use crate::entity::EntityContainer;

pub trait System {
    fn apply(&mut self, entities: &mut EntityContainer, dt: f64);
}
