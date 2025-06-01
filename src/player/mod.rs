use bevy::prelude::*;
use bevy_butler::*;
use bevy_enhanced_input::prelude::*;
use camera::FirstPersonCamera;

pub(crate) mod plugin;
pub(crate) mod camera;
pub(crate) mod weapon;

#[derive(InputContext, Component, Default)]
#[require(Transform, FirstPersonCamera, Actions<Player> = Actions::<Player>::default())]
pub struct Player;
