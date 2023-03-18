#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod biidama;
mod model;
mod update;
mod view;

fn main() {
    nannou::app(model::model)
        .update(update::update)
        .simple_window(view::view)
        .run();
}
