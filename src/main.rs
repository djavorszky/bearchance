use bevy::prelude::*;

use crate::systems::*;

mod components;
mod config;
mod debug;
mod systems;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Chance of bears".to_string(),
            width: 640.,
            height: 480.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(debug::DebugPlugin)
        .init_resource::<config::GameOpts>()
        .add_startup_system(startup::camera.system())
        .add_startup_system(startup::add_player.system())
        .add_startup_system(startup::add_entity.system())
        .add_system(movement::input_handler.system())
        .add_system(movement::movement_handler.system())
        .run();
}
