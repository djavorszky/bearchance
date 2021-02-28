mod components;
mod systems;

use crate::systems::*;
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup::camera.system())
        .add_startup_system(startup::add_player.system())
        .add_startup_system(startup::add_entity.system())
        .add_system(movement::input_handler.system())
        .add_system(movement::movement_handler.system())
        .run();
}
