use bevy::asset::RenderAssetUsages;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

#[derive(Component)]
struct CameraPosition {}

fn create_simple_parallelogram() -> Mesh {
    // Create a new mesh using a triangle list topology, where each set of 3 vertices composes a triangle.
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    // Add 4 vertices, each with its own position attribute (coordinate in
    // 3D space), for each of the corners of the parallelogram.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.0, 0.0, 0.0],
            [1.0, 2.0, 0.0],
            [2.0, 2.0, 0.0],
            [1.0, 0.0, 0.0],
        ],
    )
    // Assign a UV coordinate to each vertex.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 1.0], [0.5, 0.0], [1.0, 0.0], [0.5, 1.0]],
    )
    // Assign normals (everything points outwards)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    )
    // After defining all the vertices and their attributes, build each triangle using the
    // indices of the vertices that make it up in a counter-clockwise order.
    .with_inserted_indices(Indices::U32(vec![
        // First triangle
        0, 3, 1, // Second triangle
        1, 3, 2,
    ]))
}

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
    let cube_mesh_handle: Handle<Mesh> = meshes.add(create_simple_parallelogram());

    commands.spawn((
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial { ..default() })),
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
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
            Transform::from_xyz(0.0, 0.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
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
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_mesh)
        .add_systems(FixedUpdate, mouse_motion)
        .run();
}
