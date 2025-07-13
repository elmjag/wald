use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use std::f32::consts::TAU;

const SEGMENTS: usize = 5;

#[derive(Component)]
pub struct Branch {
    birth_time: f32,
}

impl Branch {
    pub fn new(now: f32) -> Self {
        Branch { birth_time: now }
    }

    fn age(&self, now: f32) -> f32 {
        now - self.birth_time
    }

    fn length(&self, now: f32) -> f32 {
        self.age(now) * 0.3
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

pub fn create_mesh(length: f32) -> Mesh {
    let base_scale = length.ln() / 10.0;

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
