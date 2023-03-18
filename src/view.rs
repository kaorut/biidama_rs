use nannou::prelude::*;

use super::model;

pub fn view(app: &App, model: &model::Model, frame: Frame) {
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
