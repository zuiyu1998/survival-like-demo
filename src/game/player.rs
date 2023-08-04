use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{Animation, GameState, SpriteAnimationBundle};

use super::props::Props;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "textures/player/character1.png")]
    pub green_woman: Handle<Image>,
    #[asset(path = "animation/player/player.animation.json")]
    pub animation: Handle<Animation>,
}

#[derive(Component, Default)]
pub struct PlayerCamera;

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default, LdtkEntity)]
pub struct PlayerBundle {
    pub props: Props,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    pub transform_bundle: TransformBundle,
}

impl PlayerBundle {}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), add_sprite_animate_bundle)
            .add_systems(
                Update,
                camera_fit_inside_current_level.run_if(in_state(GameState::Playing)),
            )
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Startup, setup);
    }
}
fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PlayerCamera));
}

pub fn add_sprite_animate_bundle(
    mut commands: Commands,
    query: Query<Entity, &Player>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(SpriteAnimationBundle::player(
                &player_assets,
                &mut texture_atlases,
            ));
    }
}

pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        (Without<Player>, With<PlayerCamera>),
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    let width = level.px_wid as f32;
                    let height = level.px_hei as f32;

                    orthographic_projection.viewport_origin = Vec2::ZERO;

                    orthographic_projection.scaling_mode =
                        bevy::render::camera::ScalingMode::Fixed { width, height };

                    camera_transform.translation.x =
                        (player_translation.x - level_transform.translation.x - width / 2.)
                            .clamp(0., level.px_wid as f32 - width);

                    camera_transform.translation.y =
                        (player_translation.y - level_transform.translation.y - height / 2.)
                            .clamp(0., level.px_hei as f32 - height);

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
            }
        }
    }
}
