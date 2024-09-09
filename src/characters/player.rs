use super::components::*;
// use crate::utils::PLAYER_COLLISION;
use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_rapier3d::prelude::{
    CharacterAutostep, CharacterLength, Collider, CollisionGroups,
    KinematicCharacterController, KinematicCharacterControllerOutput, Sleeping,
};
use crate::utils::PLAYER_COLLISION;

const MOUSE_SENSITIVITY: f32 = 0.4;
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
    ///Used for collisions (apparently I can't use KCC's custom shape if I want basic camera :thonk:)
    collider: Collider,
    ///Used so I don't have to manually compute collisions :P
    controller: KinematicCharacterController,
    ///Allow for custom collision groups
    coll_group: CollisionGroups,
    ///allow for enabling and disabling physics
    sleeping: Sleeping
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
                20.0, MovementStates::Walking, 2.0, Vec3::ZERO, 40.0, 50.0
            ),
            // body: RigidBody::KinematicVelocityBased,
            controller: KinematicCharacterController{
                //Add autostep so player doesn't have to jump on bumpy terrain
                autostep: Some(CharacterAutostep{
                    max_height: CharacterLength::Absolute(1.0),
                    min_width: CharacterLength::Relative(0.1),
                    include_dynamic_bodies: true
                }),
                snap_to_ground: Some(CharacterLength::Relative(0.05)),
                custom_mass: Some(2.0),
                slide: true,
                offset: CharacterLength::Relative(0.1),
                ..default()
            },
            collider: Collider::capsule_y(2.0, 1.0),
            spatial_bundle: SpatialBundle {
                transform: Transform::from_xyz(0.0, 10.0, 0.0), //Spawn the player above ground to avoid clipping
                ..default()
            },
            coll_group: PLAYER_COLLISION,
            sleeping: Sleeping::disabled(),
        }
    }
}

impl <T> CharacterBundle<T>
where T: Default + Component + Clone
{
    ///Create the default bundle with a set collision group
    pub fn from_coll_group(coll_group: CollisionGroups) -> Self {
        Self{
            coll_group,
            ..default()
        }
    }

    pub fn from_colliders(coll_group: Option<CollisionGroups>, collider: Collider) -> Self {
        Self{
            collider,
            coll_group: coll_group.unwrap_or(Default::default()),
            ..default()
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
    if key_input.just_released(KeyCode::Space) {
        y_movement += 1.0;
    }
    if key_input.just_released(KeyCode::ShiftLeft) {
        //Use shift left to toggle between running and walking
        movement.state = match movement.state {
            MovementStates::Walking => MovementStates::Sprinting,
            MovementStates::Sprinting => MovementStates::Walking
        };
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
    mut movement_query: Query<(
        &mut Movement, 
        &mut KinematicCharacterController, 
        Option<&mut KinematicCharacterControllerOutput>,
        &Transform
    ), With<Player>>,
    time: Res<Time>,
    mut jump_timer: Local<f32>,
    mut y_trans: Local<f32>
) {
    let Ok((
        mut movement,
        mut controller, 
        output, 
        trans
    )) = movement_query.get_single_mut() else {
        return;
    };
    
    let mut translation: Vec3 = 
        if let Some(mut normalized_dir) =  movement.direction.try_normalize() {
            //Only return a trans if some movement dir is set
            let speed = match movement.state {
                MovementStates::Sprinting => movement.sprint_speed,
                MovementStates::Walking => movement.base_speed
            };
            normalized_dir.y *= movement.jump_speed; //Adjust jump speed
            normalized_dir.x *= speed;
            normalized_dir.z *= speed;
            normalized_dir
        } else {
            Vec3::ZERO
        };
    let elapsed_time: f32 = time.delta_seconds();
    
    if output.map(|o| o.grounded).unwrap_or(true) {
        *jump_timer = COYOTE_TIME;
        *y_trans = 0.0; //Reset gravity because we're on the ground
    } else {
        //Reduce mobility in air for realism
        const AIR_MOBILITY: f32 = 0.2;
        translation.x *= AIR_MOBILITY;
        translation.z *= AIR_MOBILITY;
    }

    if *jump_timer > 0.0 {
        //Decrease jump timer so player can't jump freely in the air
        *jump_timer -= elapsed_time;
        if movement.direction.y > 0.0 {
            const JUMP_GRAVITY_DAMP: f32 = 5.0/6.0;
            *jump_timer = 0.0; // Can't double jump!
            *y_trans += translation.y;
            //Preemptively add to decrease gravity when jumping for stronger feel
            *y_trans += movement.gravity_scale * controller.custom_mass.expect("The player should have a mass!") * JUMP_GRAVITY_DAMP;
        }
    } else {
        movement.direction.y = 0.0; //Don't allow jumping once coyote time is over
        translation.y = 0.0;
    }

    //Add gravity 
    *y_trans -= movement.gravity_scale * controller.custom_mass.expect("The player should have a mass!");
    translation.y = *y_trans;
    controller.translation = Some(trans.rotation * (translation * elapsed_time));
}

pub(crate) fn change_perspective(
    mut player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    mut cam_query: Query<&mut Transform, With<Camera>>,
    input: Res<PerspectiveInput>
) {
    let Ok(mut player_trans) = player_query.get_single_mut() else {
        return;
    };
    player_trans.rotation = Quat::from_axis_angle(Vec3::Y, input.x.to_radians());
    
    let Ok(mut cam_trans) = cam_query.get_single_mut() else {
        return;
    };
    cam_trans.rotation = Quat::from_rotation_x(input.y.to_radians());
}
