use bevy::asset::RenderAssetUsages;
use bevy::math::u32;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use core::f32;
use std::f32::consts::{PI, TAU};

const SEGMENTS: usize = 5;
const LEAF_SPACING: f32 = 0.24;
const LENGTH_RATIO: f32 = 1.5;

use crate::assets::LoadedAssets;

#[derive(Component, Debug)]
pub struct Branch {
    birth_time: f32,
    growth_rate: f32,
    leaf_pairs: u32,
}

impl Branch {
    pub fn new(now: f32, growth_rate: f32) -> Self {
        Branch {
            birth_time: now,
            growth_rate: growth_rate,
            leaf_pairs: 0,
        }
    }

    fn age(&self, now: f32) -> f32 {
        now - self.birth_time
    }

    pub fn length(&self, now: f32) -> f32 {
        let age = self.age(now);

        let length = if age <= 1.64 {
            return age * 0.3 * LENGTH_RATIO;
        } else {
            age.ln() * LENGTH_RATIO
        };

        length * self.growth_rate
    }

    pub fn expected_leaf_pairs(&self, now: f32) -> u32 {
        ((self.length(now) - LEAF_SPACING * 0.2) / LEAF_SPACING) as u32
    }

    pub fn get_mesh(&self, now: f32) -> Mesh {
        create_mesh(self.length(now))
    }
}

fn base_vertices(num: usize, scale: f32) -> Vec<[f32; 3]> {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let step = TAU / num as f32;

    for n in 0..num {
        let angle = step * (n as f32);
        let rot = Rot2::radians(angle);

        vertices.push([rot.cos * scale, 0.0, rot.sin * scale]);
    }

    vertices
}

fn triangle_indices(num: usize) -> Indices {
    let mut indices: Vec<u32> = Vec::with_capacity(3 * num);
    let num_faces = num as u32;

    for n in 0..num_faces - 1 {
        indices.push(num_faces);
        indices.push(n + 1);
        indices.push(n);
    }
    indices.push(num_faces);
    indices.push(0);
    indices.push(num_faces - 1);

    Indices::U32(indices)
}

fn create_mesh(length: f32) -> Mesh {
    let base_scale = (length.ln() / 10.0).clamp(0.05, f32::INFINITY);

    let mut vertices = base_vertices(SEGMENTS, base_scale);
    vertices.push([0.0, length, 0.0]);

    // not sure about this, be seems to work OK
    // perhaps we need to do something more sofisticated
    // with regards to vertice normals
    let norms = vertices.clone();

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, norms)
    .with_inserted_indices(triangle_indices(SEGMENTS))
}

pub fn spawn_new(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    now: f32,
    growth_rate: f32,
    height: f32,
    y_rotation: f32,
    z_rotation: f32,
) -> Entity {
    let branch = Branch::new(now, growth_rate);
    let cube_mesh_handle: Handle<Mesh> = meshes.add(branch.get_mesh(now));

    let trans = Transform::from_xyz(0.0, height, 0.0)
        .with_rotation(Quat::from_rotation_y(y_rotation) * Quat::from_rotation_z(z_rotation));

    commands
        .spawn((
            branch,
            Mesh3d(cube_mesh_handle),
            MeshMaterial3d(materials.add(StandardMaterial { ..default() })),
            trans,
        ))
        .id()
}

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut branches: Query<(Entity, &mut Branch)>,
) {
    let now = time.elapsed_secs();

    for (entity_id, branch) in branches.iter_mut() {
        /*
         * grow branch
         *
         * replace it's old mesh with new one
         * with updated dimensions
         */
        let mesh_handle: Handle<Mesh> = meshes.add(branch.get_mesh(now));
        commands
            .entity(entity_id)
            .remove::<Mesh3d>()
            .insert(Mesh3d(mesh_handle));
    }
}

pub fn spawn_leafs(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<LoadedAssets>,
    mut branches: Query<(Entity, &mut Branch)>,
) {
    let now = time.elapsed_secs();

    let right_leaf_rot = Quat::from_rotation_z(-0.5);
    let left_leaf_rot = right_leaf_rot * Quat::from_rotation_y(PI);

    for (entity_id, mut branch) in branches.iter_mut() {
        let expected_pairs = branch.expected_leaf_pairs(now);

        while branch.leaf_pairs < expected_pairs {
            let leaf_height = ((branch.leaf_pairs + 1) as f32) * LEAF_SPACING;

            //
            // spawn 'right' leaf
            //
            let leaf = commands
                .spawn((
                    Mesh3d(assets.leaf_mesh.clone()),
                    MeshMaterial3d(assets.leaf_material.clone()),
                    Transform::from_xyz(0.0, leaf_height, 0.0).with_rotation(right_leaf_rot),
                ))
                .id();
            commands.entity(entity_id).add_child(leaf);

            //
            // spawn 'left' leaf
            //
            let leaf = commands
                .spawn((
                    Mesh3d(assets.leaf_mesh.clone()),
                    MeshMaterial3d(assets.leaf_material.clone()),
                    Transform::from_xyz(0.0, leaf_height, 0.0).with_rotation(left_leaf_rot),
                ))
                .id();
            commands.entity(entity_id).add_child(leaf);

            branch.leaf_pairs += 1;
        }
    }
}
