use bevy::prelude::*;
mod components;
mod player;
pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player::basic_movement);
    }
}

///Spawn in the player and dim the ambient light 
fn setup(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>
) {
    println!("Setup reached!");
    // Spawn a parent entity with extra components for visibility
    commands
        .spawn(player::PlayerBundle::default())
        .with_children(|player_parent| {
            //Spawn the camera and some lighting for the player to interact with the env.
            player_parent.spawn(
                (Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 5.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
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
                        transform: Transform::default().looking_at(Vec3::ZERO, Vec3::Y), 
                        ..default()
                    },
                    Name::new("Camera Light"),
                ));
        });

    println!("Dimming ambient light");
    //Dim the ambient light for spoopy :O
    ambient_light.brightness = 30.0;

    println!("Setup complete!");
}

