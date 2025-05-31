use bevy::prelude::*;
use bevy_butler::*;

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
