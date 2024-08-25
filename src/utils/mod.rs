#![allow(dead_code)] //The consts are used elsewhere

use bevy_rapier3d::geometry::{CollisionGroups, Group, SolverGroups};

pub const ENVIRONMENT_COLLISION: CollisionGroups = CollisionGroups
    ::new(Group::GROUP_1, Group::GROUP_2);

pub const ENVIRONMENT_SOLVER: SolverGroups = SolverGroups
    ::new(Group::GROUP_1, Group::GROUP_2);

pub const PLAYER_COLLISION: CollisionGroups = CollisionGroups
    ::new(Group::GROUP_2, Group::GROUP_1);

