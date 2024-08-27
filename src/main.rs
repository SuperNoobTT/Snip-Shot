mod world;
mod characters;
pub(crate) mod utils;

use bevy::prelude::{App, DefaultPlugins};
use bevy_rapier3d::{plugin::{NoUserData, RapierConfiguration}, prelude::RapierPhysicsPlugin};
use bevy_rapier3d::plugin::TimestepMode;

fn main() {
    App::new()
        .insert_resource(RapierConfiguration{
            timestep_mode: TimestepMode::Fixed {
                dt: 1.0/64.0, //bevy default of 64Hz,
                substeps: 1
            },
            ..RapierConfiguration::new(1.0)
        })
        .add_plugins((
            DefaultPlugins, 
            world::WorldPlugin,
            characters::CharactersPlugin,
            RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule()
        ))
        .run();
}
