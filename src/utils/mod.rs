#![allow(dead_code)] //The consts are used elsewhere

use bevy_rapier3d::geometry::{CollisionGroups, Group, SolverGroups};

pub const ENVIRONMENT_COLLISION: CollisionGroups = CollisionGroups
    ::new(Group::GROUP_1, Group::GROUP_2);

#[deprecated(note = "This seems to conflict with the ENVIRONMENT_COLLISION so won't be used for now")]
pub const ENVIRONMENT_SOLVER: SolverGroups = SolverGroups
    ::new(Group::GROUP_1, Group::GROUP_2);

pub const PLAYER_COLLISION: CollisionGroups = CollisionGroups
    ::new(Group::GROUP_2, Group::GROUP_1);

