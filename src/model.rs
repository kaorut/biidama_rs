use nannou::prelude::*;
use rand::prelude::*;

use super::biidama;

pub struct Model {
    pub biidamas: Vec<biidama::Biidama>,
    pub fps: usize,
}

impl Model {
    pub fn new() -> Model {
        Model {
            biidamas: Vec::new(),
            fps: 0,
        }
    }

    pub fn reset(&mut self, rect: Rect<f32>) {
        let mut rng = rand::thread_rng();

        let mut biidamas: Vec<biidama::Biidama> = Vec::new();
        let count = rng.gen_range(2u32..=50u32);
        for _ in 0..count {
            let position = Vec2::new(
                rng.gen_range(rect.left()..rect.right()),
                rng.gen_range(rect.bottom()..rect.top()),
            );
            let radius = rng.gen_range(5.0..30.0);
            let velocity = Vec2::new(rng.gen_range(0.0..5.0), rng.gen_range(0.0..5.0));
            let color = hsl(rng.gen_range(0.0..1.0), 1.0, 0.5);
            biidamas.push(biidama::Biidama {
                position,
                radius,
                velocity,
                color,
            });
        }

        self.biidamas = biidamas;
    }
}

pub fn model(app: &App) -> Model {
    let mut model = Model::new();
    model.reset(app.window_rect());
    model
}
