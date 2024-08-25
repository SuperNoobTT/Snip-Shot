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
    pub fn from_dmg(dmg: f32) -> Self {
        Self{
            dmg,
            ..Default::default()
        }
    }

    pub fn get_effects(&self) -> &Option<Vec<Effects>> {
        &self.effects
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
    base_speed: f32,
    sprint_speed: f32
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            base_speed: 50.0,
            state: MovementStates::Walking,
            direction: Vec3::ZERO,
            sprint_speed: 70.0
        }
    }
}

impl Movement {
    ///Allows the speeds to be private to prevent modifying them by accident
    pub fn get_trans(&self) -> Option<Vec3> {
        if let Some(normalized_dir) =  self.direction.try_normalize() {
            //Only return a trans if some movement dir is set
            Some(normalized_dir * match self.state {
                MovementStates::Sprinting => self.sprint_speed,
                MovementStates::Walking => self.base_speed
            })
        } else {
            None
        }
    }

    pub fn toggle_state(&mut self) -> Option<&MovementStates> {
        self.state = match self.state {
            MovementStates::Walking => MovementStates::Sprinting,
            MovementStates::Sprinting => MovementStates::Walking
        };
        Some(&self.state)
    }

    pub fn new(base_speed: f32, state: MovementStates, direction: Vec3, sprint_speed: f32) -> Self {
        Self{base_speed, state, direction, sprint_speed}
    }
}

