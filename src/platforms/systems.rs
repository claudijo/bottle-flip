use crate::physics::CustomCollisionLayer;
use crate::platforms::components::DynamicPlatform;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

// const TRASH_CAN_SIZE: Vec2 = Vec2::new(260., 320.);
// const TRASH_CAN_LID_SIZE: Vec2 = Vec2::new(296., 30.);

const FLOOR_LEVEL: f32 = -160.;

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    let window = windows.single();

    let floor_size = Vec2::new(window.size().x, window.size().y / 2. + FLOOR_LEVEL);

    commands
        .spawn((
            VisibilityBundle::default(),
            TransformBundle::from_transform(Transform::from_xyz(0., FLOOR_LEVEL, 0.)),
            RigidBody::Static,
            Collider::half_space(Vec2::Y),
            CollisionLayers::new(
                CustomCollisionLayer::Platform,
                [CustomCollisionLayer::Bottle, CustomCollisionLayer::Platform],
            ),
        ))
        .with_children(|child_builder| {
            child_builder.spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::from_size(floor_size))),
                material: materials.add(Color::srgb(0.969, 0.812, 0.569)),
                transform: Transform::from_xyz(0., -floor_size.y / 2., 0.),
                ..default()
            });
        });
}

pub fn spawn_dynamic_platforms(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_cardboard_box(&mut commands, &asset_server);
}

pub fn spawn_cardboard_box(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn((
            VisibilityBundle::default(),
            TransformBundle::from_transform(Transform::from_xyz(200., 40., 0.)),
            RigidBody::Dynamic,
            Collider::rectangle(230., 145.),
            CollisionLayers::new(
                CustomCollisionLayer::Platform,
                [CustomCollisionLayer::Bottle, CustomCollisionLayer::Platform],
            ),
            DynamicPlatform,
        ))
        .with_children(|child_builder| {
            child_builder.spawn(SpriteBundle {
                texture: asset_server.load("cardboard_box.png"),
                transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::ONE * 4.),
                ..default()
            });
        });
}
