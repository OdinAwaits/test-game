use bevy::{prelude::*, window::CursorGrabMode};
use bevy_butler::*;

use crate::player::{plugin::InputPlugin, Player, weapon::Sword};

#[add_system(schedule= Startup, plugin= InputPlugin)]
pub fn startup (
    mut commands: Commands,
    mut window: Single<&mut Window>
){
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
    commands.spawn(Player)
        .with_child(Sword);
}
