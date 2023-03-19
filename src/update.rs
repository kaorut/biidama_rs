use nannou::prelude::*;

use super::biidama;
use super::model;

pub fn update(_app: &App, model: &mut model::Model, _update: Update) {
    let frame_rect = _app.window_rect();
    let velocity_unit = _update.since_last.as_secs_f32() * 100.0;
    for b in &mut model.biidamas {
        b.position += velocity_unit * b.velocity;
        (b.position, b.velocity) = reflect_on_wall(b, frame_rect, 1.0);
    }
    if model.biidamas.len() < 2 {
        return;
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
            for _ in 0..100 {
                if !collided(&model.biidamas[i], &model.biidamas[j]) {
                    break;
                }
                model.biidamas[i].position =
                    model.biidamas[i].position + velocity_unit * model.biidamas[i].velocity;
                model.biidamas[j].position =
                    model.biidamas[j].position + velocity_unit * model.biidamas[j].velocity;
            }
        }
    }

    model.fps = (1.0 / _update.since_last.as_secs_f32()) as usize;
}

fn reflect_on_wall(biidama: &biidama::Biidama, frame_rect: Rect, k: f32) -> (Vec2, Vec2) {
    let (position_x, velocity_x) = reflect_on_wall_impl(
        biidama.position.x,
        biidama.radius,
        biidama.velocity.x,
        (frame_rect.left(), frame_rect.right()),
        k,
    );
    let (position_y, velocity_y) = reflect_on_wall_impl(
        biidama.position.y,
        biidama.radius,
        biidama.velocity.y,
        (frame_rect.bottom(), frame_rect.top()),
        k,
    );
    (
        Vec2::new(position_x, position_y),
        Vec2::new(velocity_x, velocity_y),
    )
}

fn reflect_on_wall_impl(
    position: f32,
    radius: f32,
    velocity: f32,
    walls: (f32, f32),
    k: f32,
) -> (f32, f32) {
    if position - radius < walls.0 {
        (
            2.0 * (walls.0 + radius) - position,
            if velocity < 0.0 {
                -k * velocity
            } else {
                velocity
            },
        )
    } else if position + radius > walls.1 {
        (
            2.0 * (walls.1 - radius) - position,
            if velocity > 0.0 {
                -k * velocity
            } else {
                velocity
            },
        )
    } else {
        (position, velocity)
    }
}

fn collided(biidama1: &biidama::Biidama, biidama2: &biidama::Biidama) -> bool {
    let squared_distance = (biidama1.position.x - biidama2.position.x)
        * (biidama1.position.x - biidama2.position.x)
        + (biidama1.position.y - biidama2.position.y) * (biidama1.position.y - biidama2.position.y);
    let squared_sum_of_radiuses =
        (biidama1.radius + biidama2.radius) * (biidama1.radius + biidama2.radius);
    squared_distance < squared_sum_of_radiuses
}

fn collide(biidama1: &biidama::Biidama, biidama2: &biidama::Biidama, k: f32) -> Vec2 {
    let m1 = biidama1._mass();
    let m2 = biidama2._mass();
    let v1 = biidama1.velocity;
    let v2 = biidama2.velocity;
    Vec2::new(
        collide_impl(m1, m2, v1.x, v2.x, k),
        collide_impl(m1, m2, v1.y, v2.y, k),
    )
}

fn collide_impl(m1: f32, m2: f32, v1: f32, v2: f32, k: f32) -> f32 {
    ((m1 - k * m2) * v1 + (k + 1.0) * m2 * v2) / (m1 + m2)
}
