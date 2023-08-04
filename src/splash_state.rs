use crate::AppState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Splash).continue_to_state(AppState::Menu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(AppState::Splash);
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}
