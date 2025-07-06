use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

mod tree;
use tree::create_branch;

#[derive(Component)]
struct CameraPosition {}

fn mouse_motion(
    mut evr_motion: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut Transform, With<CameraPosition>>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let mut trans = query.single_mut().unwrap();

    for ev in evr_motion.read() {
        trans.rotate_y(ev.delta.x * -0.01);
        trans.rotate_x(ev.delta.y * -0.01);
    }
}

fn setup_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh_handle: Handle<Mesh> = meshes.add(create_branch(8.2));

    commands.spawn((
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial { ..default() })),
    ));
}

fn setup_camera(
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
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 0.0, 22.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .id();

    // camera 'holder'
    let cam_holder = commands
        .spawn((CameraPosition {}, Transform::from_xyz(0.0, 0.0, 0.0)))
        .id();

    commands.entity(cam_holder).add_child(cam_id);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_mesh)
        .add_systems(FixedUpdate, mouse_motion)
        .run();
}
