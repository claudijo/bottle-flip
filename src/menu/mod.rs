mod systems;

use crate::menu::systems::{handle_restart_button, spawn_restart_button};
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_restart_button);
        app.add_systems(Update, handle_restart_button);
    }
}
