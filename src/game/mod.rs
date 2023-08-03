use bevy::prelude::*;

pub mod props;

pub mod player;

pub struct InternalGamePlugin;

impl Plugin for InternalGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(props::PropsPlugin);
    }
}
