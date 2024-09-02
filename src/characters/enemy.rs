use bevy::prelude::*;
use bevy_rapier3d::prelude::KinematicCharacterController;
use super::player::CharacterBundle;

#[derive(Component, Debug, Clone, Default)]
pub(crate) struct Enemy;

pub(crate) fn gen_enemies(
    mut commands: Commands
) {
    commands.spawn(
        CharacterBundle::<Enemy>::default()
    );
}

pub(crate) fn move_enemies() {
    todo!()
}