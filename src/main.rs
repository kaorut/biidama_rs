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
        let next_x = b.position.x + velocity_unit * b.velocity.x;
        if next_x - b.radius < frame_rect.left() {
            b.position.x = 2.0 * (frame_rect.left() + b.radius) - next_x;
            if b.velocity.x < 0.0 {
                b.velocity.x *= -1.0;
            }
        } else if next_x + b.radius > frame_rect.right() {
            b.position.x = 2.0 * (frame_rect.right() - b.radius) - next_x;
            if b.velocity.x > 0.0 {
                b.velocity.x *= -1.0;
            }
        } else {
            b.position.x = next_x;
        }

        let next_y = b.position.y + velocity_unit * b.velocity.y;
        if next_y - b.radius < frame_rect.bottom() {
            b.position.y = 2.0 * (frame_rect.bottom() + b.radius) - next_y;
            if b.velocity.y < 0.0 {
                b.velocity.y *= -1.0;
            }
        } else if next_y + b.radius > frame_rect.top() {
            b.position.y = 2.0 * (frame_rect.top() - b.radius) - next_y;
            if b.velocity.y > 0.0 {
                b.velocity.y *= -1.0;
            }
        } else {
            b.position.y = next_y;
        }
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    for b in &_model.biidamas {
        draw.ellipse()
            .x_y(b.position.x , b.position.y )
            .w_h(b.radius * 2.0, b.radius * 2.0)
            .color(b.color);
        draw.ellipse()
            .x_y(b.position.x , b.position.y )
            .w_h(10.0, 10.0)
            .color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}
