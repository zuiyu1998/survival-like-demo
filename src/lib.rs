#![allow(clippy::type_complexity)]

use bevy::app::App;
use bevy::prelude::*;
use game::InternalGamePlugin;
#[cfg(feature = "dev")]
mod dev;

mod animator;
mod common;
mod consts;
mod game;
mod splash_state;
mod ui;

mod states;

use crate::splash_state::*;
pub use animator::*;
pub use common::*;
pub use consts::*;
pub use game::*;
pub use states::*;
pub use ui::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_state::<GameState>()
            .add_plugins((
                SplashPlugin,
                InternalGamePlugin,
                MainUiPlugin,
                SpriteAnimationPlugin,
            ));

        #[cfg(feature = "dev")]
        {
            app.add_plugins(dev::DevPlugin);
        }
    }
}
