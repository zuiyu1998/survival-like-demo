use bevy::prelude::*;

use crate::{AnimationBundle, PlayerAssets, SpriteAnimationPlayer};

#[derive(Bundle, Default)]
pub struct SpriteAnimationBundle {
    pub animation_bundle: AnimationBundle,
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl SpriteAnimationBundle {
    pub fn player(
        player_assets: &PlayerAssets,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let texture_atlas = TextureAtlas::from_grid(
            player_assets.green_woman.clone(),
            Vec2::new(16.0, 28.0),
            9,
            1,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        SpriteAnimationBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 1,
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
