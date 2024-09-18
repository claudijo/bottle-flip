// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod bottle;
mod menu;
pub mod physics;
mod platforms;
mod scene;

use crate::bottle::BottlePlugin;
use crate::menu::MenuPlugin;
use crate::platforms::PlatformsPlugin;
use crate::scene::ScenePlugin;
use avian2d::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics in web builds on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bottle Flip".into(),
                        resolution: WindowResolution::new(740., 360.),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(0.671, 0.349, 0.49)))
        .add_plugins(PhysicsPlugins::default().with_length_unit(100.0))
        // .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins((ScenePlugin, BottlePlugin, PlatformsPlugin, MenuPlugin))
        .insert_resource(Gravity(Vec2::NEG_Y * 2400.0))
        .insert_resource(SubstepCount(6))
        .run();
}
