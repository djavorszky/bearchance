use bevy::ecs::FromResources;

pub type MaxSpeed = f32;

#[derive(Debug)]
pub struct GameConfig {
    pub default_max_speed: MaxSpeed,
}

impl FromResources for GameConfig {
    fn from_resources(_resources: &bevy::ecs::Resources) -> Self {
        GameConfig {
            default_max_speed: 5.,
        }
    }
}
