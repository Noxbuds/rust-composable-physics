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
        let new_x = 2.0 * self.position.x - self.old_position.x + self.acceleration.x * dt * dt;
        let new_y = 2.0 * self.position.y - self.old_position.y + self.acceleration.y * dt * dt;

        self.old_position = self.position;
        self.position = Vec2 { x: new_x, y: new_y };
        self.acceleration = Vec2 { x: 0.0, y: 0.0 };
    }

    pub fn add_acceleration(&mut self, acc: Vec2) {
        self.acceleration.x += acc.x;
        self.acceleration.y += acc.y;
    }
}
