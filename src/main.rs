use bevy::prelude::*;
mod weapon_controller;
mod input;
mod ui;
mod core;
fn main() {
    App::new()
        .add_plugins(core::Plugins)
        .add_plugins((
            input::InputPlugin,
            ui::All,
        ))
        .run();
}
