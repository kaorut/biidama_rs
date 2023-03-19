use nannou::prelude::*;

use super::biidama;

pub struct Model {
    pub biidamas: Vec<biidama::Biidama>,
    pub fps: usize,
}

impl Model {
    pub fn new() -> Model {
        Model{biidamas: Vec::new(), fps:0}
    }

    pub fn reset(&mut self) {}
}

pub fn model(_: &App) -> Model {
    // let biidamas = vec![
    //     biidama::Biidama {
    //         position: Vec2::new(0.0, 0.0),
    //         radius: 50.0,
    //         velocity: Vec2::new(5.0, 2.0),
    //         color: Rgba::new(1.0, 0.0, 0.0, 1.0),
    //     },
    //     biidama::Biidama {
    //         position: Vec2::new(120.0, 150.0),
    //         radius: 30.0,
    //         velocity: Vec2::new(10.0, -2.0),
    //         color: Rgba::new(0.0, 0.0, 1.0, 1.0),
    //     },
    //     biidama::Biidama {
    //         position: Vec2::new(30.0, 200.0),
    //         radius: 40.0,
    //         velocity: Vec2::new(-10.0, 5.0),
    //         color: Rgba::new(0.0, 0.5, 0.0, 1.0),
    //     },
    //     biidama::Biidama {
    //         position: Vec2::new(40.0, 50.0),
    //         radius: 20.0,
    //         velocity: Vec2::new(1.0, -1.0),
    //         color: Rgba::new(1.0, 0.0, 1.0, 1.0),
    //     },
    // ];
    let mut model = Model::new();
    model.reset();
    model
}
