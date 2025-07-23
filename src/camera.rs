use bevy::prelude::*;

use std::f32::consts::PI;

use bevy::input::mouse::{MouseMotion, MouseWheel};

#[derive(Component)]
pub struct CameraPosition {}

pub fn orbit(
    mut evr_motion: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut Transform, With<CameraPosition>>,
) {
    //
    // orbit camera around origin, when
    // mouse is dragged horizontally
    //
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let mut trans = query.single_mut().unwrap();

    for ev in evr_motion.read() {
        trans.rotate_y(ev.delta.x * -0.01);
    }
}

pub fn zoom(
    mut evr_wheel: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let mut trans = query.single_mut().unwrap();

    for ev in evr_wheel.read() {
        trans.translation.z -= ev.y;
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // origin position marker
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.1))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    let cam_id = commands
        .spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 22.0)))
        .id();

    // camera 'holder'

    let mut trans = Transform::from_xyz(0.0, 0.0, 0.0);
    trans.rotate_x(-PI / 3.0);

    let cam_holder = commands.spawn((CameraPosition {}, trans)).id();

    commands.entity(cam_holder).add_child(cam_id);
}
