use crate::bottle::components::{
    Bottle, BottleContent, BottleContentJoint, GrabAnchor, GrabJoint, Grabbable,
};
use crate::bottle::resources::TouchGrab;
use crate::physics::CustomCollisionLayer;
use avian2d::prelude::*;
use bevy::input::touch::TouchPhase;
use bevy::prelude::*;

const BOTTLE_BODY_SIZE: Vec2 = Vec2::new(50., 90.);
const BOTTLE_NECK_HEIGHT: f32 = 30.;
const BOTTLE_CAP_SIZE: Vec2 = Vec2::new(20., 10.);
const BOTTLE_DENSITY: f32 = 0.4;
const CONTENT_RADIUS: f32 = 18.;
const CONTENT_DENSITY: f32 = 4.;

pub fn spawn_bottle(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(0., 0., 0.)),
        RigidBody::Kinematic,
        GrabAnchor,
    ));

    let container = commands
        .spawn((
            VisibilityBundle::default(),
            TransformBundle::default(),
            ColliderDensity(BOTTLE_DENSITY),
            RigidBody::Dynamic,
            Collider::rectangle(BOTTLE_BODY_SIZE.x, BOTTLE_BODY_SIZE.y),
            CollisionLayers::new(
                CustomCollisionLayer::Bottle,
                [CustomCollisionLayer::Platform],
            ),
            Bottle,
            Grabbable,
            AngularDamping(0.5),
        ))
        .with_children(|child_builder| {
            // Bottle body
            child_builder.spawn(SpriteBundle {
                texture: asset_server.load("plastic_bottle.png"),
                transform: Transform::from_xyz(0., 15., 0.).with_scale(Vec3::ONE * 4.),
                ..default()
            });

            // Bottleneck
            child_builder.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    0.,
                    BOTTLE_BODY_SIZE.y / 2.,
                    0.,
                )),
                Grabbable,
                ColliderDensity(BOTTLE_DENSITY),
                Collider::triangle(
                    Vec2::Y * BOTTLE_NECK_HEIGHT,
                    Vec2::new(-BOTTLE_BODY_SIZE.x / 2., 0.),
                    Vec2::new(BOTTLE_BODY_SIZE.x / 2., 0.),
                ),
                CollisionLayers::new(
                    CustomCollisionLayer::Bottle,
                    [CustomCollisionLayer::Platform],
                ),
            ));
            // Bottle cap
            child_builder.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    0.,
                    BOTTLE_BODY_SIZE.y / 2. + BOTTLE_NECK_HEIGHT - BOTTLE_CAP_SIZE.y / 2.,
                    0.,
                )),
                Grabbable,
                ColliderDensity(BOTTLE_DENSITY),
                Collider::rectangle(BOTTLE_CAP_SIZE.x, BOTTLE_CAP_SIZE.y),
                CollisionLayers::new(
                    CustomCollisionLayer::Bottle,
                    [CustomCollisionLayer::Platform],
                ),
            ));
        })
        .id();

    let content_1 = commands
        .spawn((
            TransformBundle::from_transform(Transform::from_xyz(0., CONTENT_RADIUS, 0.)),
            ColliderDensity(CONTENT_DENSITY),
            RigidBody::Dynamic,
            Collider::circle(CONTENT_RADIUS),
            CollisionLayers::new(
                CustomCollisionLayer::Content,
                [CustomCollisionLayer::Content],
            ),
            BottleContent,
        ))
        .id();

    let content_2 = commands
        .spawn((
            TransformBundle::from_transform(Transform::from_xyz(0., -CONTENT_RADIUS, 0.)),
            ColliderDensity(CONTENT_DENSITY),
            RigidBody::Dynamic,
            Collider::circle(CONTENT_RADIUS),
            CollisionLayers::new(
                CustomCollisionLayer::Content,
                [CustomCollisionLayer::Content],
            ),
            BottleContent,
        ))
        .id();

    commands.spawn((
        PrismaticJoint::new(container, content_1)
            .with_free_axis(Vec2::Y)
            .with_limits(
                -BOTTLE_BODY_SIZE.y / 2. + CONTENT_RADIUS,
                BOTTLE_BODY_SIZE.y / 2. - CONTENT_RADIUS + BOTTLE_NECK_HEIGHT,
            ),
        BottleContentJoint,
    ));

    commands.spawn((
        PrismaticJoint::new(container, content_2)
            .with_free_axis(Vec2::Y)
            .with_limits(
                -BOTTLE_BODY_SIZE.y / 2. + CONTENT_RADIUS,
                BOTTLE_BODY_SIZE.y / 2. - CONTENT_RADIUS + BOTTLE_NECK_HEIGHT,
            ),
        BottleContentJoint,
    ));
}

fn world_from_viewport(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    position: Option<Vec2>,
) -> Option<Vec2> {
    let viewport_position = position?;

    // Calculate a world position based on the cursor's position.
    camera.viewport_to_world_2d(camera_transform, viewport_position)
}

fn try_grab_bottle(
    commands: &mut Commands,
    anchor: Entity,
    bottle: Entity,
    bottle_transform: &GlobalTransform,
    cursor_position: Vec2,
    grabbable_transform: &GlobalTransform,
    grabbable: &Collider,
) -> bool {
    let (_scale, rotation, translation) = grabbable_transform.to_scale_rotation_translation();
    if grabbable.contains_point(translation.xy(), rotation, cursor_position) {
        let grabbed_at = bottle_transform
            .affine()
            .inverse()
            .transform_point(cursor_position.extend(0.));

        commands.spawn((
            RevoluteJoint::new(anchor, bottle)
                .with_local_anchor_2(grabbed_at.xy())
                .with_angular_velocity_damping(20.),
            GrabJoint,
        ));

        return true;
    }

    false
}

pub fn grab_bottle_using_touch(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    anchor_query: Query<Entity, With<GrabAnchor>>,
    bottle_query: Query<(Entity, &GlobalTransform), With<Bottle>>,
    grabbable_query: Query<(&GlobalTransform, &Collider), With<Grabbable>>,
    touches: Res<Touches>,
    mut touch_grab: ResMut<TouchGrab>,
) {
    if touch_grab.0.is_some() {
        return;
    }

    for touch in touches.iter() {
        if touches.just_pressed(touch.id()) {
            let (camera, camera_transform) = camera_query.single();
            let Some(cursor_position) =
                world_from_viewport(camera, camera_transform, Some(touch.position()))
            else {
                return;
            };

            for anchor in &anchor_query {
                for (bottle, bottle_transform) in &bottle_query {
                    for (grabbable_transform, collider) in &grabbable_query {
                        if try_grab_bottle(
                            &mut commands,
                            anchor,
                            bottle,
                            bottle_transform,
                            cursor_position,
                            grabbable_transform,
                            collider,
                        ) {
                            touch_grab.0 = Some(touch.id());
                            return;
                        }
                    }
                }
            }
        }
    }
}

pub fn grab_bottle_using_mouse(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    anchor_query: Query<Entity, With<GrabAnchor>>,
    bottle_query: Query<(Entity, &GlobalTransform), With<Bottle>>,
    grabbable_query: Query<(&GlobalTransform, &Collider), With<Grabbable>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        let window = windows.single();
        let Some(cursor_position) =
            world_from_viewport(camera, camera_transform, window.cursor_position())
        else {
            return;
        };

        for anchor in &anchor_query {
            for (bottle, bottle_transform) in &bottle_query {
                for (grabbable_transform, collider) in &grabbable_query {
                    if try_grab_bottle(
                        &mut commands,
                        anchor,
                        bottle,
                        bottle_transform,
                        cursor_position,
                        grabbable_transform,
                        collider,
                    ) {
                        return;
                    }
                }
            }
        }
    }
}

pub fn drag_bottle_using_touch(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut anchor_query: Query<&mut Transform, With<GrabAnchor>>,
    touch_grab: Res<TouchGrab>,
    mut touch_event_reader: EventReader<TouchInput>,
) {
    if touch_grab.0.is_none() {
        return;
    }

    for touch_event in touch_event_reader.read() {
        if TouchPhase::Moved == touch_event.phase {
            if touch_grab.0 != Some(touch_event.id) {
                continue;
            }

            let (camera, camera_transform) = camera_query.single();
            let Some(cursor_point) =
                world_from_viewport(camera, camera_transform, Some(touch_event.position))
            else {
                return;
            };

            for mut anchor_transform in &mut anchor_query {
                anchor_transform.translation = cursor_point.extend(0.);
            }

            return;
        }
    }
}

pub fn drag_bottle_using_mouse(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut anchor_query: Query<&mut Transform, With<GrabAnchor>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = windows.single();
    let Some(cursor_point) =
        world_from_viewport(camera, camera_transform, window.cursor_position())
    else {
        return;
    };

    for mut anchor_transform in &mut anchor_query {
        anchor_transform.translation = cursor_point.extend(0.);
    }
}

pub fn release_bottle_using_touch(
    mut commands: Commands,
    joint_query: Query<Entity, With<GrabJoint>>,
    mut touch_grab: ResMut<TouchGrab>,
    mut touch_event_reader: EventReader<TouchInput>,
) {
    if touch_grab.0.is_none() {
        return;
    }

    for touch_event in touch_event_reader.read() {
        if touch_event.phase == TouchPhase::Ended || touch_event.phase == TouchPhase::Canceled {
            if touch_grab.0 != Some(touch_event.id) {
                continue;
            }

            for joint in &joint_query {
                commands.entity(joint).despawn();
            }

            touch_grab.0 = None;
            return;
        }
    }
}

pub fn release_bottle_using_mouse(
    mut commands: Commands,
    joint_query: Query<Entity, With<GrabJoint>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_released(MouseButton::Left) {
        for joint in &joint_query {
            commands.entity(joint).despawn();
        }
    }
}
