#![allow(dead_code)] //Components will be used elsewhere!

use bevy::prelude::{Component, Vec3};

#[derive(Component, Debug, Clone)]
///Handles most health related things
///TODO: May want to add stuff like debuffs later on (or use a seperate component!)
pub(crate) struct Health {
    pub max: f32,
    pub curr: f32, 
    pub overflow: f32
}

impl Default for Health {
    fn default() -> Self {
        Health {
            max: 10.0,
            curr: 10.0,
            overflow: 0.0
        }
    }
}

#[derive(Clone, Debug)]
///Unfilled enum for possible attack effects, e.g. bleed, poision, etc.
/// TODO: Add some effects!
pub(crate) enum Effects {

}

#[derive(Component, Debug, Clone, Default)]
///Handle attack related stuff
pub(crate) struct Attack {
    pub dmg: f32,
    ///Attack may or may not have additional effects, it may have more than one!
    effects: Option<Vec<Effects>>,
    sfx: Option<f32> //TODO: Deal with this when implementing sound later !
}

impl Attack {
    #[inline(always)]
    pub const fn from_dmg(dmg: f32) -> Self {
        Self{
            dmg,
            effects: None,
            sfx: None
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum MovementStates {
    #[default]
    Walking,
    Sprinting
}

#[derive(Component, Debug, Clone)]
///Handles movement with immutable speeds and a mutable movement state & direction vector
pub(crate) struct Movement {
    pub direction: Vec3,
    pub state: MovementStates,
    pub gravity_scale: f32,
    pub base_speed: f32,
    pub sprint_speed: f32,
    pub jump_speed: f32
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            base_speed: 50.0,
            state: MovementStates::Walking,
            gravity_scale: 1.0,
            direction: Vec3::ZERO,
            sprint_speed: 70.0,
            jump_speed: 100.0
        }
    }
}

impl Movement {
    pub fn new(base_speed: f32, state: MovementStates, gravity_scale: f32, direction: Vec3, sprint_speed: f32, jump_speed: f32) -> Self {
        Self{base_speed, state, gravity_scale, direction, sprint_speed, jump_speed}
    }
}

