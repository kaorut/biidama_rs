use nannou::prelude::*;

pub struct Biidama {
    pub position: Vec2,
    pub radius: f32,
    pub velocity: Vec2,
    pub color: Hsl,
}

impl Biidama {
    pub fn _mass(&self) -> f32 {
        (self.radius * self.radius).sqrt()
    }
}
