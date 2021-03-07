use std::fs;

use bevy::ecs::FromResources;
use serde::Deserialize;
use serde_yaml;

#[derive(Debug, Deserialize)]
pub struct GameOpts {
    pub player: PlayerOpts,
    pub debug: DebugOpts,
}

#[derive(Debug, Deserialize)]
pub struct PlayerOpts {
    pub speed: f32,
}

#[derive(Debug, Deserialize)]
pub struct DebugOpts {
    pub log_fps: bool,
    pub log_options_at_startup: bool,
}

impl FromResources for GameOpts {
    fn from_resources(_resources: &bevy::ecs::Resources) -> Self {
        let config = fs::read_to_string("config.yml").expect("Failed reading config file");

        serde_yaml::from_str(config.as_str()).expect("failed parsing configuration")
    }
}
