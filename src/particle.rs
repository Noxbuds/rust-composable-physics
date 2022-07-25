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
    pub fn update_position(&mut self, dt: f64) {
        // verlet integration
        let new_position = self.position * 2.0 - self.old_position + self.acceleration * dt * dt;

        self.old_position = self.position;
        self.position = new_position;
        self.acceleration = Vec2 { x: 0.0, y: 0.0 };
    }

    pub fn add_acceleration(&mut self, acc: Vec2) {
        self.acceleration += acc;
    }
}
