use crate::utils::Vec2;

#[derive(Clone, Copy)]
pub struct Particle {
    pub old_position: Vec2,
    pub position: Vec2,
    pub acceleration: Vec2,
    pub radius: f64,
    pub color: [f32; 4],
}

impl Particle {
    pub fn add_acceleration(&mut self, acc: Vec2) {
        self.acceleration += acc;
    }
}
