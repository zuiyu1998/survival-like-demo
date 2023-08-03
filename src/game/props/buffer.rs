use bevy::prelude::*;
use std::{marker::PhantomData, time::Duration};

use super::Props;
use crate::GameState;

#[derive(Debug, Default)]
pub struct BufferPlugin<B>(PhantomData<B>);

impl<B: Buffer + Component> Plugin for BufferPlugin<B> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_add::<B>.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Debug, Default)]
pub struct TimerBufferPlugin<B>(PhantomData<B>);

impl<B: TimerBuffer + Component> Plugin for TimerBufferPlugin<B> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (on_add::<B>, auto_delete::<B>).run_if(in_state(GameState::Playing)),
        );
    }
}

fn on_add<B: Component + Buffer>(mut query: Query<(&B, &mut Props), Added<B>>) {
    for (buffer, mut props) in query.iter_mut() {
        buffer.on_add(&mut props);
    }
}

fn auto_delete<B: Component + TimerBuffer>(
    mut commands: Commands,
    mut query: Query<(Entity, &mut B, &mut Props)>,
    time: Res<Time>,
) {
    for (entity, mut buffer, mut props) in query.iter_mut() {
        buffer.tick(time.delta());

        if buffer.just_finished() {
            buffer.on_delete(&mut props);
            commands.entity(entity).remove::<B>();
        }
    }
}

pub trait Buffer: Send + Sync + 'static {
    fn on_add(&self, props: &mut Props);
    fn on_delete(&self, props: &mut Props);
}

pub trait TimerBuffer: Buffer {
    fn tick(&mut self, delta: Duration);
    fn just_finished(&self) -> bool;
}
