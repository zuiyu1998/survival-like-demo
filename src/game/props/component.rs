use super::{Buffer, PropsClass};
use bevy::prelude::*;

#[derive(Debug, Component, Default)]
pub struct VelocityUp;

impl Buffer for VelocityUp {
    fn on_add(&self, props: &mut super::Props) {
        props.set_max_value(PropsClass::Velocity, 20);
    }

    fn on_delete(&self, props: &mut super::Props) {
        props.set_max_value(PropsClass::Velocity, -20);
    }
}
