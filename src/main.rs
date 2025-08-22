use bevy::prelude::*;

use std::time::Duration;

use bevy::time::common_conditions::on_timer;

mod assets;
mod tree;
use tree::branch;

mod camera;
use camera::CameraPlugin;

const TREE_UPDATE_TIME: u64 = 1200;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin))
        .add_systems(Startup, (tree::setup, assets::setup))
        .add_systems(
            Update,
            (tree::update, branch::update, branch::spawn_leafs)
                .run_if(on_timer(Duration::from_millis(TREE_UPDATE_TIME))),
        )
        .run();
}
