use crate::{entity::Entity, utils::Vec2};
use super::Component;

pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn to_vec2(&self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }

    pub fn from_vec2(old: Vec2) -> Position {
        Position { x: old.x, y: old.y }
    }
}

impl Component<Position> for Position {
    fn attach(entity: &mut Entity, component: Position, id: i32) {
        entity.properties_f64.insert(Self::get_property_key(id, 0), component.x);
        entity.properties_f64.insert(Self::get_property_key(id, 1), component.y);
    }

    fn get(entity: &Entity, id: i32) -> Option<Position> {
        let maybe_x = entity.properties_f64.get(&Self::get_property_key(id, 0));
        let maybe_y = entity.properties_f64.get(&Self::get_property_key(id, 1));

        if let (Some(x), Some(y)) = (maybe_x, maybe_y) {
            Some(Position {
                x: *x,
                y: *y
            })
        } else {
            None
        }
    }

    fn get_property_key(id: i32, field: i32) -> i32 {
        id * 2 + field
    }
}
