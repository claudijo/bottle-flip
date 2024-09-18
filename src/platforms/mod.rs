pub mod components;
pub mod systems;

use crate::platforms::systems::{spawn_dynamic_platforms, spawn_ground};
use bevy::prelude::*;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ground, spawn_dynamic_platforms));
    }
}
