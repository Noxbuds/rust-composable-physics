use crate::{component::Component, entity::Entity};

pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Component<Position> for Position {
    fn attach(entity: &mut Entity, component: Position) {
        entity.properties_f64.insert("position.x".to_string(), component.x);
        entity.properties_f64.insert("position.y".to_string(), component.y);
    }

    fn get(entity: &Entity) -> Option<Position> {
        let maybe_x = entity.properties_f64.get("position.x");
        let maybe_y = entity.properties_f64.get("position.y");

        if let (Some(x), Some(y)) = (maybe_x, maybe_y) {
            Some(Position {
                x: *x,
                y: *y
            })
        } else {
            None
        }
    }
}
