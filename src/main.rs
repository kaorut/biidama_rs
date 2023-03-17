use nannou::prelude::*;

struct Biidama {
    position: Vec2,
    radius: f32,
    velocity: Vec2,
    color: Rgba,
}

impl Biidama {
    fn _mass(&self) -> f32 {
        return self.radius;
    }
}

struct Model {
    biidamas: Vec<Biidama>,
    fps: usize,
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
        color: Rgba::new(1.0, 0.0, 0.0, 1.0),
    });
    biidamas.push(Biidama {
        position: Vec2::new(120.0, 150.0),
        radius: 30.0,
        velocity: Vec2::new(20.0, -5.0),
        color: Rgba::new(0.0, 0.0, 1.0, 1.0),
    });
    return Model {
        biidamas: biidamas,
        fps: 0,
    };
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let frame_rect = _app.window_rect();
    let velocity_unit = _update.since_last.as_secs_f32() * 100.0;
    for b in &mut model.biidamas {
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
    for i in 0..model.biidamas.len() - 1 {
        for j in i + 1..model.biidamas.len() {
            let v1: Vec2;
            let v2: Vec2;
            {
                let b1 = &model.biidamas[i];
                let b2 = &model.biidamas[j];
                if !collided(b1, b2) {
                    continue;
                }
                v1 = collide(b1, b2, 1.0);
                v2 = collide(b2, b1, 1.0);
            }
            model.biidamas[i].velocity = v1;
            model.biidamas[j].velocity = v2;
        }
    }

    model.fps = (1.0 / _update.since_last.as_secs_f32()) as usize;
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

fn collided(biidama1: &Biidama, biidama2: &Biidama) -> bool {
    let squared_distance = (biidama1.position.x - biidama2.position.x)
        * (biidama1.position.x - biidama2.position.x)
        + (biidama1.position.y - biidama2.position.y) * (biidama1.position.y - biidama2.position.y);
    let squared_sum_of_radiuses =
        (biidama1.radius + biidama2.radius) * (biidama1.radius + biidama2.radius);
    return squared_distance < squared_sum_of_radiuses;
}

fn collide(biidama1: &Biidama, biidama2: &Biidama, k: f32) -> Vec2 {
    let m1 = biidama1._mass();
    let m2 = biidama2._mass();
    let v1 = biidama1.velocity;
    let v2 = biidama2.velocity;

    return Vec2::new(
        collide_impl(m1, m2, v1.x, v2.x, k),
        collide_impl(m1, m2, v1.y, v2.y, k),
    );
}

fn collide_impl(m1: f32, m2: f32, v1: f32, v2: f32, k: f32) -> f32 {
    return ((m1 - k * m2) * v1 + (k + 1.0) * m2 * v2) / (m1 + m2);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window_rect = app.window_rect();
    draw.background().color(WHITE);

    for b in &model.biidamas {
        draw.ellipse()
            .x_y(b.position.x, b.position.y)
            .w_h(b.radius * 2.0, b.radius * 2.0)
            .color(b.color);
    }

    let text_area = Rect::from_w_h(window_rect.w(), 24.0).top_left_of(window_rect.pad(0.0));
    draw.text(std::format!("FPS: {}", model.fps).as_str())
        .color(BLACK)
        .left_justify()
        .xy(text_area.xy())
        .wh(text_area.wh())
        .finish();

    draw.to_frame(app, &frame).unwrap();
}
