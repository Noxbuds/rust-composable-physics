use std::collections::HashMap;

pub type EntityContainer = HashMap<i32, Entity>;

pub struct Entity {
    pub id: i32,
    pub properties_i32: HashMap<i32, i32>,
    pub properties_f64: HashMap<i32, f64>,
    pub properties_bool: HashMap<i32, bool>,
}

impl Entity {
    pub fn new(id: i32) -> Entity {
        Entity {
            id: id,
            properties_i32: HashMap::new(),
            properties_f64: HashMap::new(),
            properties_bool: HashMap::new()
        }
    }
}
