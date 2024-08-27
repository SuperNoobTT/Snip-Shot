use super::components::*;
use crate::utils::PLAYER_COLLISION;
use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_rapier3d::prelude::{
    CharacterAutostep, CharacterLength, Collider, GravityScale,
    KinematicCharacterControllerOutput, KinematicCharacterController, RigidBody,
};

const MOUSE_SENSITIVITY: f32 = 0.3;
const COYOTE_TIME: f32 = 0.2;
#[derive(Resource, Default, Clone, Debug, Deref, DerefMut)]
pub(crate) struct PerspectiveInput(Vec2);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) struct AfterInput;

#[derive(Component, Clone, Debug, Default)]
pub struct Player;

#[derive(Bundle, Clone, Debug)]
pub struct CharacterBundle<T> 
where T: Component + Clone + Default
{
    ///Not really used, just used to identify that the entity is a player using a With<> query
    marker: T,
    ///Handle health (see component)
    health: Health,
    ///Handle attack (see component)
    attack: Attack,
    ///Handles movement, a little more involved (see component)
    movement: Movement,
    ///Used to give children an inheritable visibility and transform
    spatial_bundle: SpatialBundle,
    ///For rapier3d to physics! (almost always RigidBody::Dyanmic)
    body: RigidBody,
    ///Tbh idk how to use this lol
    gravity_scale: GravityScale,
    ///Used so I don't have to manually compute collisions :P
    controller: KinematicCharacterController
}

impl<T> Default for CharacterBundle<T> 
where T: Component + Clone + Default
{
    fn default() -> Self {
        Self {
            marker: T::default(), 
            health: Health {
                curr: 10.0,
                max: 10.0,
                overflow: 5.0
            },
            attack: Attack::from_dmg(5.0),
            movement: Movement::new(
                80.0, MovementStates::Walking, Vec3::ZERO, 120.0, 150.0
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
                custom_shape: Some((Collider::cuboid(5.0, 40.0, 5.0), Vec3::ZERO, Quat::default())),
                custom_mass: Some(50.0),
                filter_groups: Some(PLAYER_COLLISION),
                ..default()
            },
            gravity_scale: GravityScale(2.0), //Double gravity for hacky gravity modifying :P
            spatial_bundle: SpatialBundle {
                transform: Transform::from_xyz(0.0, 80.0, 0.0), //Spawn the player above ground to avoid clipping
                ..default()
            }
        }
    }
}

pub(crate) fn handle_input(
    mut movement_query: Query<&mut Movement, With<Player>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut mouse_input: EventReader<MouseMotion>,
    mut perspective_input: ResMut<PerspectiveInput>,
) {

    let Ok(mut movement) = movement_query.get_single_mut() else {
        return;
    };

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
    if key_input.just_released(KeyCode::ShiftLeft) {
        //Use shift left to toggle between running and walking
        movement.toggle_state();
    }
    if key_input.just_released(KeyCode::ShiftRight) {
        //Reset mouse camera movementÍ
        perspective_input.0 = Vec2::new(0.0, 0.0);
    }

    //Update the movement component for movement handling later on!
    movement.direction = Vec3::new(x_movement, y_movement, z_movement);

    for mouse_movement in mouse_input.read() {
        perspective_input.x -= mouse_movement.delta.x * MOUSE_SENSITIVITY;
        perspective_input.y -= mouse_movement.delta.y * MOUSE_SENSITIVITY;
        //Limit the y movement so player doesn't break their back!
        perspective_input.y = perspective_input.y.clamp(-89.9, 89.9);
    }
}

pub(crate) fn move_player(
    mut movement_query: Query<(&mut Movement, &mut KinematicCharacterController, &GravityScale, Option<&mut KinematicCharacterControllerOutput>), With<Player>>,
    time: Res<Time>,
    mut jump_timer: Local<f32>,
    mut gravity: Local<f32>
) {
    let Ok((mut movement, mut controller, grav_scale, output)) = movement_query.get_single_mut() else {
        return;
    };

    let elapsed_time: f32 = time.delta_seconds();

    // if output.map(|o| o.grounded).unwrap_or(false) {
    //     *jump_timer = COYOTE_TIME;
    //     *gravity = 0.0; //Reset gravity because we're on the ground
    // }

    if let Some(stuff) = output {
        dbg!(&stuff.effective_translation);
        if stuff.grounded {
            *jump_timer = COYOTE_TIME;
            *gravity = 0.0; //Reset gravity because we're on the ground
        }
    }

    if *jump_timer > 0.0 {
        //Decrease jump timer so player can't jump freely in the air
        *jump_timer -= elapsed_time;
        if movement.direction.y > 0.0 {
            *jump_timer = 0.0; // Can't double jump!
            //Decrease gravity when jumping for stronger feel
            *gravity -= grav_scale.0 * elapsed_time * controller.custom_mass.expect("The player should have a mass!")/2.0;
        }
    } else {
        println!("Coyote time over");
        movement.direction.y = 0.0; //Don't allow jumping once coyote time is over
    }

    controller.translation = match movement.get_trans() {
        Some(mut trans) => {
            //Add gravity
            trans.y -= *gravity;
            //Return the transform, adjusted by delta time
            Some(trans * elapsed_time)
        },
        None => {
            //Just fall if no other movement to handle
            Some(Vec3::new(0.0, *gravity * -1.0, 0.0))
        }
    };

    //Increase the 'gravity' to match expected acceleration feel
    *gravity += grav_scale.0 * elapsed_time * controller.custom_mass.expect("The player should have a mass!");
}

pub(crate) fn change_perspective(
    mut player_query: Query<&mut Transform, (With<Player>, With<KinematicCharacterController>, Without<Camera3d>)>,
    mut cam_query: Query<&mut Transform, With<Camera3d>>,
    input: Res<PerspectiveInput>
) {
    let Ok(mut player_trans) = player_query.get_single_mut() else {
        return;
    };
    player_trans.rotation = Quat::from_axis_angle(Vec3::Y, input.x.to_radians());

    let Ok(mut cam_trans) = cam_query.get_single_mut() else {
        return;
    };
    cam_trans.rotation = Quat::from_axis_angle(Vec3::X, input.y.to_radians());
}

pub fn check_pos(
    player: Query<&Transform, With<Player>>
) {
    for plyr in player.iter() {
        dbg!(plyr.translation);
    }
}