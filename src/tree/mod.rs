use bevy::prelude::*;
pub mod branch;
use branch::Branch;

use rand::SeedableRng;
use rand::rngs::SmallRng;

mod angles;
use angles::new_branch_angle;

const RND_SEED: u64 = 0;

#[derive(Component)]
pub struct Tree {
    branch_angles: Vec<f32>,
    rng: SmallRng,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            branch_angles: vec![],
            rng: SmallRng::seed_from_u64(RND_SEED),
        }
    }

    pub fn get_new_branch_angle(&mut self) -> f32 {
        new_branch_angle(&self.branch_angles, &mut self.rng)
    }
}

fn maybe_add_branch(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    now: f32,
    tree: &mut Tree,
    trunk: &Branch,
) {
    let trunk_length = trunk.length(now);
    let expected_children = (trunk_length / 0.69) as usize;

    while expected_children > tree.branch_angles.len() {
        let new_branch_angle = tree.get_new_branch_angle();

        tree.branch_angles.push(new_branch_angle);

        branch::spawn_new(
            commands,
            meshes,
            materials,
            now,
            0.38,
            trunk_length * 0.58,
            new_branch_angle,
            0.5,
        );
    }
}

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut trees: Query<(&mut Tree, &Children)>,
    branches: Query<&Branch>,
) {
    let now = time.elapsed_secs();

    for (mut tree, children) in trees.iter_mut() {
        for child in children.iter() {
            let trunk = branches.get(child).unwrap();
            maybe_add_branch(
                &mut commands,
                &mut meshes,
                &mut materials,
                now,
                &mut tree,
                trunk,
            );
        }
    }
}

pub fn setup(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let now = time.elapsed_secs();

    let trunk = branch::spawn_new(
        &mut commands,
        &mut meshes,
        &mut materials,
        now,
        1.0,
        0.0,
        0.0,
        0.0,
    );

    let tree = commands.spawn((Tree::new(),)).id();

    commands.entity(tree).add_child(trunk);
}
