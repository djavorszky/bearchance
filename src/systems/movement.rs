use crate::components::*;

use bevy::prelude::*;

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
    mut query: Query<&mut Force, With<Player>>,
) {
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
