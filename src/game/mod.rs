use bevy::prelude::*;

pub mod props;

pub mod loading;
pub mod map;
pub mod player;

pub use loading::*;
pub use map::*;
pub use player::*;
pub use props::*;

pub struct InternalGamePlugin;

impl Plugin for InternalGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PropsPlugin, PlayerPlugin, LoadingPlugin, MapPlugin));
    }
}
