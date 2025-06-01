use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_butler::*;
use bevy_enhanced_input::prelude::*;

use crate::{core::Settings, player::plugin::InputPlugin};

use super::camera::CameraSensitivity;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct EnterWeaponMotionMode;

#[add_observer(plugin= InputPlugin)]
fn enter_weapon_motion_mode (
    t: Trigger<Started<EnterWeaponMotionMode>>,
    mut sens: ResMut<CameraSensitivity>,
    mut commands: Commands,
){
    info!("Entered WeaponMotionMode");
    sens.0 = 0.1;
}

#[add_observer(plugin= InputPlugin)]
fn leave_weapon_motion_mode (
    t: Trigger<Completed<EnterWeaponMotionMode>>,
    mut sens: ResMut<CameraSensitivity>,
    mut commands: Commands,
){
    info!("Left WeaponMotionMode");
    sens.0 = 1.0;
}

#[derive(Component, InputContext)]
#[require(Transform,)]
pub struct SwordInputContext;

#[derive(Resource)]
pub struct SwordAsset(Handle<Scene>);
#[derive(Component)]
#[require(
    Transform = Transform::from_xyz(0.0,0.0,-1.0),
    RigidBody::Kinematic,
    GravityScale = GravityScale(0.0),
    Mass = Mass(1800.0),
    Actions<SwordInputContext> = Actions::<SwordInputContext>::default(),
)]
pub struct Sword;
#[add_observer(plugin= WeaponPlugin)]
fn on_sword_add(
    t: Trigger<OnAdd, Sword>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sword_handle: Option<ResMut<SwordAsset>>,
){
    info!("Sword Added");
    let handle = if let Some(ref mut handle_res) = sword_handle {
        handle_res.0.clone()
    } else {
        let handle = asset_server.load("simple-longsword.glb#Scene0");
        commands.insert_resource(SwordAsset(handle.clone()));
        handle
    };

    commands.entity(t.target())
        .insert(SceneRoot(handle));
}

#[derive(Default)]
#[butler_plugin]
pub struct WeaponPlugin;

#[add_system(schedule= Startup, plugin= WeaponPlugin)]
fn setup(
    mut commands: Commands,
) {
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 100_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Debug, InputAction)]
#[input_action(output= bool)]
struct StartWeaponRotation;
#[derive(Debug, InputAction)]
#[input_action(output= bool)]
struct StartWeaponThrust;
#[derive(Debug, InputAction)]
#[input_action(output= bool)]
struct StartCombinedMotion;
#[derive(Debug, InputAction)]
#[input_action(output= Vec2)]
struct WeaponRotation;
#[derive(Debug, InputAction)]
#[input_action(output= Vec2)]
struct WeaponThrust;
#[derive(Debug, InputAction)]
#[input_action(output= Vec2)]
struct WeaponCombined;
#[add_observer(plugin= WeaponPlugin)]
fn bind_weapon_actions(
    t: Trigger<Binding<SwordInputContext>>,
    mut sword: Query<&mut Actions<SwordInputContext>>,
    settings: Res<Settings>,
){
    let mut actions = sword.get_mut(t.target()).unwrap();
    actions
        .bind::<StartWeaponRotation>().to(MouseButton::Left)
        ;
    actions
        .bind::<StartWeaponThrust>().to(MouseButton::Right)
        ;
    actions
        .bind::<StartCombinedMotion>().with_conditions((
            Chord::<StartWeaponRotation>::default(),
            Chord::<StartWeaponThrust>::default(),
        ));
    actions
        .bind::<WeaponRotation>().to(
            Input::mouse_motion()
                .with_modifiers((
                    Scale::splat(settings.mouse_sensitivity),
                    Negate::all()
                ))
        )
            .with_conditions(Chord::<StartWeaponRotation>::default());
    actions
        .bind::<EnterWeaponMotionMode>()
            .with_conditions(Chord::<StartWeaponRotation>::default())
            .with_conditions(Chord::<StartWeaponThrust>::default());
}

#[add_observer(plugin= WeaponPlugin)]
fn rotate_weapon(
    t: Trigger<Fired<WeaponRotation>>,
    mut q: Query<&mut Transform, With<Sword>>,
){
    info!("Rotating weapon...");
    let mut transform = q.get_mut(t.target()).unwrap();
    let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

    yaw += t.value.x.to_radians();
    pitch += t.value.y.to_radians();

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

#[add_observer(plugin= WeaponPlugin)]
fn motion_started(
    t: Trigger<Fired<EnterWeaponMotionMode>>
){
    info!("Entered WeaponMotionMode");
}
#[add_observer(plugin= WeaponPlugin)]
fn test_weapon_rotation_bool  (t: Trigger<Fired<StartWeaponRotation>>) {
    info!("StartWeaponRotation was fired");
}
