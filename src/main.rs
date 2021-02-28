mod components;

use crate::components::*;
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(add_player.system())
        .add_startup_system(add_entity.system())
        .add_system(input_handler.system())
        .add_system(movement_handler.system())
        .run();
}

fn movement_handler(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Force, &mut Transform)>,
) {
    for (mut velocity, mut force, mut transform) in query.iter_mut() {
        if force.0.length() != 0.0 {
            velocity.0 += force.0;

            force.0 = Vec3::zero();
        }

        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn input_handler(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Force, With<Player>>) {
    for mut force in query.iter_mut() {
        let mut new_force = Vec3::default();
        if keyboard_input.pressed(KeyCode::Left) {
            new_force.x -= 30.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            new_force.y += 30.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            new_force.x += 30.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            new_force.y -= 30.0;
        }

        force.0 += new_force;
    }
}

fn setup(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}

fn add_player(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
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

fn add_entity(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
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
