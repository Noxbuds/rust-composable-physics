use crate::{entity::Entity};
use super::Component;

pub struct CircleCollider {
    pub radius: f64,
}

impl Component<CircleCollider> for CircleCollider {
    fn attach(entity: &mut Entity, component: CircleCollider, id: i32) {
        let radius_key = Self::get_property_key(id, 0);
        entity.properties_f64.insert(radius_key, component.radius);
    }

    fn get(entity: &Entity, id: i32) -> Option<CircleCollider> {
        let radius_key = Self::get_property_key(id, 0);
        let maybe_radius = entity.properties_f64.get(&radius_key);

        if let Some(radius) = maybe_radius {
            Some(CircleCollider { radius: *radius })
        } else {
            None
        }
    }

    fn get_property_key(id: i32, field: i32) -> i32 {
        200 + id * 10 + field
    }
}
