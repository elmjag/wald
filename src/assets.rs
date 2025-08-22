use bevy::prelude::*;

const LEAF: &str = "leaf.glb";

#[derive(Resource)]
pub struct LoadedAssets {
    pub leaf_mesh: Handle<Mesh>,
    pub leaf_material: Handle<StandardMaterial>,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let leaf_handle = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset(LEAF),
    );

    let mat_handle: Handle<StandardMaterial> = asset_server.load(
        GltfAssetLabel::Material {
            index: 0,
            is_scale_inverted: false,
        }
        .from_asset(LEAF),
    );

    commands.insert_resource(LoadedAssets {
        leaf_mesh: leaf_handle,
        leaf_material: mat_handle,
    });
}
