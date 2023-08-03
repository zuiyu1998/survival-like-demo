use crate::{Animation, GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        )
        .add_collection_to_loading_state::<_, PlayerAssets>(GameState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "textures/player/character1.png")]
    pub green_woman: Handle<Image>,
    #[asset(path = "animation/player/player.animation.json")]
    pub animation: Handle<Animation>,
}
