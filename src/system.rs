use crate::entity::Entity;

pub trait System {
    fn apply(&self, entities: &mut Vec<Entity>, dt: f64);
}
