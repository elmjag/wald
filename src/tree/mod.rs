use bevy::prelude::*;

mod branch;
use branch::Branch;

pub fn setup(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh_handle: Handle<Mesh> = meshes.add(branch::create_mesh(1.2));

    let now = time.elapsed_secs();

    commands.spawn((
        Branch::new(now),
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial { ..default() })),
    ));
}

pub fn update(
    time: Res<Time>,
    mut commands: Commands,
    query_enemy: Query<(Entity, &Branch)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let now = time.elapsed_secs();

    for (entity_id, branch) in query_enemy.iter() {
        let mesh_handle: Handle<Mesh> = meshes.add(branch.get_mesh(now));
        commands
            .entity(entity_id)
            .remove::<Mesh3d>()
            .insert(Mesh3d(mesh_handle));
    }
}
