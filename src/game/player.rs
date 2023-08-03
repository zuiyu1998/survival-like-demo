use bevy::prelude::*;

use crate::{AnimationBundle, GameState, SpriteAnimationPlayer};

use super::{
    loading::PlayerAssets,
    props::{Props, VelocityUp},
};

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub props: Props,
    pub velocity_up: VelocityUp,
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub animation_bundle: AnimationBundle,
}

impl PlayerBundle {
    pub fn new(player_assets: &PlayerAssets, texture_atlases: &mut Assets<TextureAtlas>) -> Self {
        let texture_atlas = TextureAtlas::from_grid(
            player_assets.green_woman.clone(),
            Vec2::new(16.0, 28.0),
            9,
            1,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        PlayerBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                ..Default::default()
            },
            animation_bundle: AnimationBundle {
                animation_handle: player_assets.animation.clone(),
                animation_player: SpriteAnimationPlayer {
                    animate_name: "idle".to_owned(),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (spwner_player,));
    }
}

fn spwner_player(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(PlayerBundle::new(&player_assets, &mut texture_atlases));
}
