use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TouchGrab(pub Option<u64>);
