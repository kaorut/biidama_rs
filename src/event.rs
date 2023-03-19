use nannou::prelude::*;

use super::model;

pub fn event(app: &App, model: &mut model::Model, event: Event) {
    if let Event::WindowEvent {
        id: _,
        simple: Some(MouseReleased(MouseButton::Left)),
    } = event
    {
        model.reset(app.window_rect());
    }
}
