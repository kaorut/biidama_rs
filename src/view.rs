use nannou::prelude::*;

use crate::biidama;

use super::model;

pub fn view(app: &App, model: &model::Model, frame: Frame) {
    let draw = app.draw();
    let window_rect = app.window_rect();
    draw.background().color(WHITE);

    for b in &model.biidamas {
        paint(&draw, b)
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

fn paint(draw: &Draw, biidama: &biidama::Biidama) {
    paint_impl(draw, biidama, 1.0, biidama.color.lightness / 4.0);
    paint_impl(draw, biidama, 0.9, biidama.color.lightness / 2.0);
    paint_impl(draw, biidama, 0.8, biidama.color.lightness);
    paint_impl(
        draw,
        biidama,
        0.6,
        0.5 + (1.0 - biidama.color.lightness) / 4.0,
    );
    paint_impl(
        draw,
        biidama,
        0.4,
        0.5 + (1.0 - biidama.color.lightness) / 2.0,
    );
    paint_impl(draw, biidama, 0.2, 1.0);
}

fn paint_impl(draw: &Draw, biidama: &biidama::Biidama, rate: f32, lightness: f32) {
    let position = Vec2::new(
        biidama.position.x - biidama.radius * (1.0 - rate) / 2.0 / 2.0.sqrt(),
        biidama.position.y + biidama.radius * (1.0 - rate) / 2.0 / 2.0.sqrt(),
    );
    let radius = biidama.radius * rate;
    let color = Hsl::new(biidama.color.hue, biidama.color.saturation, lightness);
    draw.ellipse()
        .x_y(position.x, position.y)
        .w_h(radius * 2.0, radius * 2.0)
        .color(color);
}
