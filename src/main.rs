mod world;
mod characters;
pub(crate) mod utils;
use bevy::prelude::{App, DefaultPlugins};
use bevy_rapier3d::{plugin::NoUserData, prelude::RapierPhysicsPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, 
            world::WorldPlugin,
            characters::CharactersPlugin,
            RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule()
        ))
        .run();
}

