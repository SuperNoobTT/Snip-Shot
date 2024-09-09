use bevy::prelude::*;
pub mod components;
pub mod player;
mod enemy;
use player::*;
pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PerspectiveInput>()
            .init_resource::<enemy::PositionRandomiser>()
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, handle_input)
            .configure_sets(FixedUpdate, AfterInput.after(handle_input)) //Create a set for everything that runs after input handling
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
                Camera3dBundle{
                    transform: Transform::from_xyz(0.0, 0.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                }
            );

            player_parent.spawn((
                SpotLightBundle {
                    spot_light: SpotLight{
                        outer_angle: std::f32::consts::FRAC_PI_3,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.5, 0.0).looking_to(Vec3::ZERO, Vec3::Y),
                    ..default()
                },
                Name::new("Camera Light"),
            ));
        });

    //Dim the ambient light for spoopy :O
    ambient_light.brightness = 5.0;
}

