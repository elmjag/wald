use bevy::prelude::*;

use std::time::Duration;

use bevy::time::common_conditions::on_timer;

mod camera;
mod tree;

const TREE_UPDATE_TIME: u64 = 2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, camera::setup)
        .add_systems(Startup, tree::setup)
        .add_systems(FixedUpdate, (camera::orbit, camera::zoom))
        .add_systems(
            Update,
            tree::update.run_if(on_timer(Duration::from_secs(TREE_UPDATE_TIME))),
        )
        .run();
}
