
use bevy::prelude::*;
use bevy_butler::*;


pub struct InputPlugin;
#[butler_plugin]
impl bevy::app::Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_input_context::<Player>()
            .add_input_context::<FirstPersonCamera>()
        ;
    }
}
