use crate::entity::Entity;

pub mod position;
pub mod acceleration;
pub mod circle_collider;

pub trait Component<T> {
    fn attach(entity: &mut Entity, component: T, id: i32);
    fn get(entity: &Entity, id: i32) -> Option<T>;
    fn get_property_key(id: i32, field: i32) -> i32;
}
