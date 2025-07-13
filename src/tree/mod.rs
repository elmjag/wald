use bevy::prelude::*;

mod branch;
use branch::Branch;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh_handle: Handle<Mesh> = meshes.add(branch::create_mesh(1.2));

    commands.spawn((
        Branch {},
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial { ..default() })),
    ));
}

pub fn update(
    time: Res<Time>,
    mut commands: Commands,
    query_enemy: Query<Entity, With<Branch>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for entity_id in query_enemy.iter() {
        let mesh_handle: Handle<Mesh> = meshes.add(branch::create_mesh(time.elapsed_secs() / 3.0));
        commands
            .entity(entity_id)
            .remove::<Mesh3d>()
            .insert(Mesh3d(mesh_handle));
    }
}
