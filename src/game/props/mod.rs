use bevy::{prelude::*, utils::HashMap};

pub mod buffer;
pub mod component;

pub use buffer::*;
pub use component::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Reflect)]
pub enum PropsClass {
    Velocity,
    Other,
}

#[derive(Debug, Reflect)]
pub struct PropValue {
    class: PropsClass,
    max: f32,
    value: f32,
    min: f32,
}

impl PropValue {
    pub fn velocity(value: f32) -> Self {
        PropValue {
            class: PropsClass::Velocity,
            max: value,
            value: value,
            min: value,
        }
    }

    pub fn set_max_value(&mut self, value: f32) {
        self.max += value;
    }

    pub fn set_value(&mut self, value: f32) {
        self.value += value;
    }

    pub fn value(&self) -> i32 {
        let target = self.value.min(self.max).max(self.min);

        target as i32
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Props(HashMap<PropsClass, PropValue>);

impl Default for Props {
    fn default() -> Self {
        let mut default = HashMap::default();

        default.insert(PropsClass::Velocity, PropValue::velocity(40.0));

        Self(default)
    }
}

impl Props {
    pub fn set_prop_value(&mut self, class: PropsClass, value: i32) {
        if let Some(prop) = self.0.get_mut(&class) {
            prop.set_value(value as f32);
        }
    }

    pub fn set_max_value(&mut self, class: PropsClass, value: i32) {
        if let Some(prop) = self.0.get_mut(&class) {
            prop.set_value(value as f32);
        }
    }
}

pub struct PropsPlugin;

impl Plugin for PropsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BufferPlugin::<VelocityUp>::default(),))
            .register_type::<Props>();
    }
}
