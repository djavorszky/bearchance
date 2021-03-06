use bevy::ecs::FromResources;
use serde::Deserialize;
use serde_json;

use std::fs;

#[derive(Debug, Deserialize)]
pub struct GameOpts {
    pub player_speed: f32,
}

impl FromResources for GameOpts {
    fn from_resources(_resources: &bevy::ecs::Resources) -> Self {
        let config = fs::read_to_string("config.json").expect("Failed reading config file");

        serde_json::from_str(config.as_str()).expect("failed parsing configuration")
    }
}
