pub mod components;
mod resources;
pub mod systems;

use crate::bottle::resources::TouchGrab;
use crate::bottle::systems::{
    drag_bottle_using_mouse, drag_bottle_using_touch, grab_bottle_using_mouse,
    grab_bottle_using_touch, release_bottle_using_mouse, release_bottle_using_touch, spawn_bottle,
};
use bevy::prelude::*;

pub struct BottlePlugin;

impl Plugin for BottlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TouchGrab::default());

        app.add_systems(Startup, spawn_bottle);
        app.add_systems(
            Update,
            (
                grab_bottle_using_mouse,
                grab_bottle_using_touch,
                drag_bottle_using_mouse,
                drag_bottle_using_touch,
                release_bottle_using_mouse,
                release_bottle_using_touch,
            ),
        );
    }
}
