use crate::{entity::Entity, utils::Vec2};

use super::Component;

pub struct Acceleration {
    pub x: f64,
    pub y: f64,
}

impl Acceleration {
    pub fn accelerate(entity: &mut Entity, direction: Vec2) {
        let maybe_acc = Acceleration::get(entity, 0);
        if let Some(acc) = maybe_acc {
            let new_acc = Acceleration {
                x: acc.x + direction.x,
                y: acc.y + direction.y,
            };

            Acceleration::attach(entity, new_acc, 0);
        }
    }
}

impl Component<Acceleration> for Acceleration {
    fn attach(entity: &mut Entity, component: Acceleration, id: i32) {
        let x_key = Self::get_property_key(id, 0);
        let y_key = Self::get_property_key(id, 1);

        entity.properties_f64.insert(x_key, component.x);
        entity.properties_f64.insert(y_key, component.y);
    }

    fn get(entity: &Entity, id: i32) -> Option<Acceleration> {
        let x_key = Self::get_property_key(id, 0);
        let y_key = Self::get_property_key(id, 1);

        let maybe_x = entity.properties_f64.get(&x_key);
        let maybe_y = entity.properties_f64.get(&y_key);

        if let (Some(x), Some(y)) = (maybe_x, maybe_y) {
            Some(Acceleration {
                x: *x,
                y: *y
            })
        } else {
            None
        }
    }

    fn get_property_key(id: i32, field: i32) -> i32 {
        100 + id * 2 + field
    }
}
