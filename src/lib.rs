#![allow(clippy::type_complexity)]

#[cfg(feature = "dev")]
mod dev;

mod splash;

use crate::splash::SplashPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    // Splash
    #[default]
    Splash,
    // During this State the actual game logic is executed
    InGame,
    // ui
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>().add_plugins((SplashPlugin,));

        #[cfg(feature = "dev")]
        {
            app.add_plugins(dev::DevPlugin);
        }
    }
}
