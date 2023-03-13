use nannou::prelude::*;

struct Biidama {
    position: Vec2,
    radius: f32,
    velocity: Vec2,
    color: Rgba,
}

struct Model {
    biidamas: Vec<Biidama>,
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let mut biidamas = Vec::new();
    biidamas.push(Biidama {
        position: Vec2::new(0.0, 0.0),
        radius: 50.0,
        velocity: Vec2::new(1.0, 0.0),
        color: Rgba::new(1.0, 0.0, 0.0, 0.5),
    });
    biidamas.push(Biidama {
        position: Vec2::new(20.0, 50.0),
        radius: 30.0,
        velocity: Vec2::new(0.0, -1.0),
        color: Rgba::new(0.0, 0.0, 1.0, 0.5),
    });
    return Model { biidamas: biidamas };
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let velocity_unit = _update.since_last.as_secs_f32() * 100.0;
    for b in &mut _model.biidamas {
        b.position.x += velocity_unit * b.velocity.x;
        b.position.y += velocity_unit * b.velocity.y;
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    for b in &_model.biidamas {
        draw.ellipse()
            .x_y(b.position.x - b.radius, b.position.y - b.radius)
            .w_h(b.radius * 2.0, b.radius * 2.0)
            .color(b.color);
    }

    draw.to_frame(app, &frame).unwrap();
}
