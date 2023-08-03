use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(Debug, SystemParam)]
pub struct StateController<'w> {
    app_state: ResMut<'w, NextState<AppState>>,
    game_state: ResMut<'w, NextState<GameState>>,
}

impl<'a> StateController<'a> {
    pub fn start_game(&mut self) {
        self.app_state.set(AppState::InGame);
        self.game_state.set(GameState::Playing);
    }
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    // Splash
    #[default]
    Splash,
    // During this State the actual game logic is executed
    InGame,
    // ui
    Menu,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // Splash
    #[default]
    None,
    Loading,
    Playing,
}
