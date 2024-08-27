use bevy::prelude::*;
mod components;
mod player;
use player::*;
pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PerspectiveInput>()
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, handle_input)
            .configure_sets(FixedUpdate, AfterInput.after(handle_input)) //Create a set for everything that runs after input handling
            .add_systems(FixedUpdate, check_pos)
            .add_systems(FixedUpdate, (move_player, change_perspective).in_set(AfterInput));
    }
}

///Spawn in the player and dim the ambient light 
fn setup(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>
) {
    // Spawn a parent entity with extra components for visibility
    commands
        .spawn(CharacterBundle::<Player>::default())
        .with_children(|player_parent| {
            //Spawn the camera and some lighting for the player to interact with the env.
            player_parent.spawn(
                (Camera3dBundle {
                    //Position the camera above the player for realistic head positioning
                    transform: Transform::from_xyz(0.0, 1.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                }, 
                VisibilityBundle::default()
            ));

            player_parent.spawn((
                    SpotLightBundle {
                        spot_light: SpotLight{
                            inner_angle: 1.0,
                            outer_angle: 1.5,
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 40.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
                        ..default()
                    },
                    Name::new("Camera Light"),
                ));
        });

    //Dim the ambient light for spoopy :O
    ambient_light.brightness = 40.0;
}

