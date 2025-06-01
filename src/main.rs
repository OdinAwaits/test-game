use bevy::prelude::*;
mod player;
mod core;
mod scenes;
fn main() {
    App::new()
        .add_plugins(core::Plugins)
        .add_plugins((
            player::plugin::InputPlugin,
            core::ui::All,
        ))
        .run();
}

