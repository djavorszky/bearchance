use bevy::prelude::*;

use crate::{components::*, config::GameOpts};

pub fn movement_handler(
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

pub fn input_handler(
    keyboard_input: Res<Input<KeyCode>>,
    opts: Res<GameOpts>,
    mut query: Query<&mut Force, With<Player>>,
) {
    let input_force = opts.player.speed;

    for mut force in query.iter_mut() {
        let mut new_force = Vec3::zero();

        for btn in keyboard_input.get_just_pressed() {
            match btn {
                KeyCode::A => new_force.x -= input_force,
                KeyCode::D => new_force.x += input_force,
                KeyCode::W => new_force.y += input_force,
                KeyCode::S => new_force.y -= input_force,
                _ => (),
            }
        }

        for btn in keyboard_input.get_just_released() {
            match btn {
                KeyCode::A => new_force.x += input_force,
                KeyCode::D => new_force.x -= input_force,
                KeyCode::W => new_force.y -= input_force,
                KeyCode::S => new_force.y += input_force,
                _ => (),
            }
        }

        force.0 += new_force;
    }
}
