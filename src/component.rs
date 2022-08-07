use crate::entity::Entity;

pub trait Component<T> {
    fn attach(entity: &mut Entity, component: T);
    fn get(entity: &Entity) -> Option<T>;
}
