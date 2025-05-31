use bevy::prelude::*;
use bevy_butler::*;

mod plugin;
mod camera;
mod weapon;

#[derive(InputContext, Component, Default)]
#[require(Transform, FirstPersonCamera, Actions<Player> = Actions::<Player>::default())]
pub struct Player;
