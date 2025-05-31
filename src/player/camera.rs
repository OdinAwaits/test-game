use bevy::prelude::*;
use bevy_butler::*;


#[derive(Component, InputContext, Default)]
#[require(Transform, Camera3d, Actions<FirstPersonCamera> = Actions::<FirstPersonCamera>::default())]
pub struct FirstPersonCamera;

#[derive(Resource, Default)]
#[insert_resource(plugin= InputPlugin, init= CameraSensitivity(1.0))]
pub struct CameraSensitivity(f32);

#[derive(Debug, InputAction)]
#[input_action(output= Vec2)]
struct CameraRotation;

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

