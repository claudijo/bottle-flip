use crate::scene::systems::spawn_camera;
use bevy::prelude::*;

pub mod systems;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}
