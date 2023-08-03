use bevy::prelude::*;

use crate::GameState;

use super::props::{Props, VelocityUp};

#[derive(Debug, Bundle, Default)]
pub struct PlayerBundle {
    props: Props,
    velocity_uo: VelocityUp,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (spwner_player,));
    }
}

fn spwner_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
}
