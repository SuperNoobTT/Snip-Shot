use super::components::*;
use crate::utils::PLAYER_COLLISION;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    CharacterAutostep, CharacterLength, Collider,
    KinematicCharacterControllerOutput, KinematicCharacterController, RigidBody,
};

#[derive(Component, Clone, Debug, Default)]
pub struct Player;

#[derive(Bundle, Clone, Debug)]
pub struct PlayerBundle {
    ///Not really used, just used to identify that the entity is a player using a With<> query
    marker: Player,
    health: Health,
    attack: Attack,
    movement: Movement,
    spatial_bundle: SpatialBundle,
    body: RigidBody,
    controller: KinematicCharacterController
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player::default(), 
            health: Health {
                curr: 10.0,
                max: 10.0,
                overflow: 5.0
            },
            attack: Attack::from_dmg(5.0),
            movement: Movement::new(
                80.0, MovementStates::Walking, Vec3::ZERO, 120.0
            ),
            body: RigidBody::Dynamic,
            controller: KinematicCharacterController{
                //Add autostep so player doesn't have to jump on bumpy terrain
                autostep: Some(CharacterAutostep{
                    max_height: CharacterLength::Absolute(1.0),
                    min_width: CharacterLength::Relative(0.1),
                    include_dynamic_bodies: true
                }),
                snap_to_ground: Some(CharacterLength::Relative(0.2)),
                custom_shape: Some((Collider::cuboid(20.0, 40.0, 20.0), Vec3::new(0.0, 80.0, 0.0), Quat::default())),
                custom_mass: Some(50.0),
                filter_groups: Some(PLAYER_COLLISION),
                ..default()
            },
            spatial_bundle: SpatialBundle::default()
        }
    }
}

pub(crate) fn basic_movement(
    mut movement_query: Query<(&mut KinematicCharacterController, &mut Movement, Option<&KinematicCharacterControllerOutput>)>,
    key_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {

    for (mut controller, mut movement, controller_output) in movement_query.iter_mut() {
        let mut x_movement = 0.0;
        let mut y_movement = 0.0;
        let mut z_movement = 0.0;

        // Horizontal (x-axis) movement
        if key_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            x_movement -= 1.0;
        }
        if key_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            x_movement += 1.0;
        }

        // Vertical (y-axis) movement
        if key_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            z_movement -= 1.0;
        }
        if key_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            z_movement += 1.0;
        }

        // Up/Down (z-axis) movement
        if key_input.pressed(KeyCode::Space) {
            y_movement += 1.0;
        }
        if key_input.pressed(KeyCode::ShiftLeft) {
            movement.toggle_state();
        }

        movement.direction = Vec3::new(x_movement, y_movement, z_movement);

        //Tell the controller to move by the calculated trans
        controller.translation = movement.get_trans().map_or(None, |trans| Some(trans*time.delta_seconds()))
    }
}
