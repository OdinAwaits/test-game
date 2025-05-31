use bevy_enhanced_input::prelude::*;
use bevy_butler::*;
use crate::{core::Settings, weapon_controller::Sword};
use crate::weapon_controller::SwordInputContext;
#[add_plugin(to_plugin= InputPlugin)]
use crate::weapon_controller::WeaponPlugin;
use bevy::{prelude::*, window::CursorGrabMode};

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

#[derive(Component, InputContext, Default)]
#[require(Transform, Camera3d, Actions<FirstPersonCamera> = Actions::<FirstPersonCamera>::default())]
pub struct FirstPersonCamera;

#[derive(Resource, Default)]
#[insert_resource(plugin= InputPlugin, init= CameraSensitivity(1.0))]
pub struct CameraSensitivity(f32);


#[derive(InputContext, Component, Default)]
#[require(Transform, FirstPersonCamera, Actions<Player> = Actions::<Player>::default())]
pub struct Player;

#[derive(Debug, InputAction)]
#[input_action(output= Vec2)]
struct CameraRotation;

#[derive(Debug, InputAction)]
#[input_action(output= bool)]
struct WeaponMotionMode;

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

#[add_observer(plugin= InputPlugin)]
fn bind_camera_actions (
    trigger: Trigger<Binding<FirstPersonCamera>>,
    mut players: Query<&mut Actions<FirstPersonCamera>>,
    camera_sens: Res<CameraSensitivity>,
    settings: Res<Settings>,
){
    let mut actions = players.get_mut(trigger.target()).unwrap();
    actions
        .bind::<CameraRotation>().to((
            Input::mouse_motion()
                .with_modifiers((Scale::splat(settings.mouse_sensitivity), Negate::all())),
        ));
    actions
        .bind::<WeaponMotionMode>().to((
            MouseButton::Left,
            MouseButton::Right,
        ));
}

#[add_observer(plugin= InputPlugin)]
fn apply_camera_rotation(
    t: Trigger<Fired<CameraRotation>>,
    mut camera: Query<&mut Transform, With<FirstPersonCamera>>,
    sens: Res<CameraSensitivity>
){
    if sens.0 == 0.0 {return;}


    let mut transform = camera.get_mut(t.target()).unwrap();
    let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

    yaw += t.value.x.to_radians() * sens.0;
    pitch += t.value.y.to_radians() * sens.0;

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

#[add_observer(plugin= InputPlugin)]
fn enter_weapon_motion_mode (
    t: Trigger<Started<WeaponMotionMode>>,
    mut sens: ResMut<CameraSensitivity>,
    mut commands: Commands,
){
    sens.0 = 0.1;
    commands
        .entity(t.target())
        .insert(Actions::<SwordInputContext>::default());
}
#[add_observer(plugin= InputPlugin)]
fn leave_weapon_motion_mode (
    t: Trigger<Completed<WeaponMotionMode>>,
    mut sens: ResMut<CameraSensitivity>,
    mut commands: Commands,
){
    sens.0 = 1.0;
    commands
        .entity(t.target())
        .remove::<Actions<SwordInputContext>>();
}
