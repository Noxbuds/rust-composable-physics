use std::collections::HashMap;

pub struct Entity {
    pub id: i32,
    pub properties_i32: HashMap<String, i32>,
    pub properties_f64: HashMap<String, f64>,
    pub properties_bool: HashMap<String, bool>,
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
