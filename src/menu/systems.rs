use crate::bottle::components::{Bottle, BottleContent, BottleContentJoint, GrabAnchor};
use crate::bottle::systems::spawn_bottle;
use crate::platforms::components::DynamicPlatform;
use crate::platforms::systems::spawn_cardboard_box;
use bevy::prelude::*;

pub fn spawn_restart_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("restart_button_sprite.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(17), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                left: Val::Px(32.),
                right: Val::Px(32.),
                top: Val::Px(32.),
                ..default()
            },
            ..default()
        })
        .with_children(|child_builder| {
            child_builder
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(17. * 4.),
                        height: Val::Px(17. * 4.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|child_builder| {
                    child_builder.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Px(17. * 4.),
                                height: Val::Px(17. * 4.),
                                ..default()
                            },
                            image: UiImage::new(texture_handle),
                            ..default()
                        },
                        TextureAtlas::from(texture_atlas_handle),
                    ));
                });
        });
}

pub fn handle_restart_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    bottle_query: Query<Entity, With<Bottle>>,
    bottle_content_query: Query<Entity, With<BottleContent>>,
    bottle_content_joint_query: Query<Entity, With<BottleContentJoint>>,
    grab_anchor_query: Query<Entity, With<GrabAnchor>>,
    dynamic_platform_query: Query<Entity, With<DynamicPlatform>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut atlas_images: Query<&mut TextureAtlas>,
) {
    let mut reset_game = false;
    for interaction in &mut interaction_query {
        for mut atlas_image in &mut atlas_images {
            match *interaction {
                Interaction::Pressed => {
                    atlas_image.index = 1;
                    reset_game = true;
                }
                Interaction::Hovered => {}
                Interaction::None => {
                    atlas_image.index = 0;
                }
            }
        }
    }

    // Quick and dirty reset solution. Should use game state transitions
    if reset_game {
        for bottle in &bottle_query {
            commands.entity(bottle).despawn_recursive();
        }

        for grab_anchor in &grab_anchor_query {
            commands.entity(grab_anchor).despawn();
        }

        for bottle_content in &bottle_content_query {
            commands.entity(bottle_content).despawn();
        }

        for bottle_content_joint in &bottle_content_joint_query {
            commands.entity(bottle_content_joint).despawn();
        }

        for dynamic_platform in &dynamic_platform_query {
            commands.entity(dynamic_platform).despawn_recursive();
        }

        spawn_cardboard_box(&mut commands, &asset_server);
        spawn_bottle(commands, asset_server);
    }
}
