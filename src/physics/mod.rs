use avian2d::prelude::*;

#[derive(PhysicsLayer)]
pub enum CustomCollisionLayer {
    Bottle,  // Layer 0
    Content, // Layer 1
    Platform,
}
