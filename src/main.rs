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
        velocity: Vec2::new(2.0, 1.0),
        color: Rgba::new(1.0, 0.0, 0.0, 0.5),
    });
    biidamas.push(Biidama {
        position: Vec2::new(20.0, 50.0),
        radius: 30.0,
        velocity: Vec2::new(20.0, -1.0),
        color: Rgba::new(0.0, 0.0, 1.0, 0.5),
    });
    return Model { biidamas: biidamas };
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let frame_rect = _app.window_rect();
    let velocity_unit = _update.since_last.as_secs_f32() * 100.0;
    for b in &mut _model.biidamas {
        (b.position.x, b.velocity.x) = reflect_on_wall(
            b.position.x,
            b.radius,
            b.velocity.x,
            velocity_unit,
            frame_rect.left(),
            frame_rect.right(),
        );
        (b.position.y, b.velocity.y) = reflect_on_wall(
            b.position.y,
            b.radius,
            b.velocity.y,
            velocity_unit,
            frame_rect.bottom(),
            frame_rect.top(),
        );
    }
}

fn reflect_on_wall(
    position: f32,
    radius: f32,
    velocity: f32,
    velocity_unit: f32,
    frame_first: f32,
    frame_last: f32,
) -> (f32, f32) {
    let next_position = position + velocity_unit * velocity;
    if next_position - radius < frame_first {
        return (
            2.0 * (frame_first + radius) - next_position,
            if velocity < 0.0 { -velocity } else { velocity },
        );
    } else if next_position + radius > frame_last {
        return (
            2.0 * (frame_last - radius) - next_position,
            if velocity > 0.0 { -velocity } else { velocity },
        );
    } else {
        return (next_position, velocity);
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    for b in &_model.biidamas {
        draw.ellipse()
            .x_y(b.position.x, b.position.y)
            .w_h(b.radius * 2.0, b.radius * 2.0)
            .color(b.color);
        draw.ellipse()
            .x_y(b.position.x, b.position.y)
            .w_h(10.0, 10.0)
            .color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}
