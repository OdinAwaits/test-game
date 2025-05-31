use bevy_butler::*;
use bevy::{prelude::*, render::{settings::{Backends, WgpuSettings}, RenderPlugin}, window::PresentMode};
use avian3d::PhysicsPlugins;
use bevy_enhanced_input::EnhancedInputPlugin;
#[add_plugin( to_plugin= Plugins)]
use bevy_skein::SkeinPlugin;

pub struct Plugins;

#[butler_plugin]
impl Plugin for Plugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }.into(),
                    ..default()
                })

                .set(WindowPlugin {
                    primary_window: Some(
                        Window{
                            present_mode: PresentMode::Immediate,
                            ..default()
                        }),
                    ..default()
            }))
            .add_plugins((
                PhysicsPlugins::new(FixedUpdate),
                EnhancedInputPlugin,
            ));

    }
}

#[derive(Resource)]
#[insert_resource(plugin= Plugins)]
pub struct Settings {
    pub mouse_sensitivity: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings{
            mouse_sensitivity: 0.1,
        }
    }
}

