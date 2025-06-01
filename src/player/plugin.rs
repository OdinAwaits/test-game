use bevy_enhanced_input::prelude::*;
use bevy::prelude::*;
use bevy_butler::*;
#[add_plugin(to_plugin= InputPlugin)]
use super::weapon::WeaponPlugin;

use super::{camera::FirstPersonCamera, Player};


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
