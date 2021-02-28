use crate::components::*;
use bevy::prelude::*;

pub fn camera(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}

pub fn add_player(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let transform = Transform::from_translation(Vec3::new(0.0, -50.0, 1.0));
    let material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    let sprite = Sprite::new(Vec2::new(30.0, 30.0));

    commands
        .spawn((Player, Force(Vec3::default()), Velocity(Vec3::default())))
        .with_bundle(SpriteBundle {
            material,
            transform,
            sprite,
            ..Default::default()
        });
}

pub fn add_entity(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let pos = Vec3::new(-150.0, -30.0, 1.0);
    commands
        .spawn((GameEntity, Force(Vec3::zero()), Velocity(Vec3::zero())))
        .with_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
            transform: Transform::from_translation(pos),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        });
}
