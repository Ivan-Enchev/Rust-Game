use crate::structs::*;
use bevy::prelude::*;
use bevy_retrograde::prelude::*;
use std::time::Instant; 

fn sprite_movement (start_point: Vec3, end_point: Vec3) -> Vec3 {
    let mut direction = Vec3::new(0., 0., 0.);

        if start_point.x > end_point.x {
            direction += Vec3::new(-1.0, 0., 0.);
        }

        if start_point.x < end_point.x {
            direction += Vec3::new(1.0, 0., 0.);
        }

        if start_point.y > end_point.y {
            direction += Vec3::new(0., -1.0, 0.);
        }

        if start_point.y < end_point.y {
            direction += Vec3::new(0., 1.0, 0.);
        }

    direction = direction.normalize_or_zero();
    return direction;
}

pub fn move_slime(mut query: Query<(&mut Velocity, &mut GlobalTransform, &Speed), (With<Slime>, Without<Player>)>, 
    player_query: Query<&GlobalTransform, (With<Player>, Without<Slime>)>) {
    for (mut velocity, slime_pos, speed) in query.iter_mut() {
        for player_pos in player_query.iter() {
            let direction = sprite_movement(slime_pos.translation, player_pos.translation);
            *velocity = Velocity::from_linear(direction * speed.value);
        }

    }
}

pub fn move_flame_spirit(mut query: Query<(&mut Velocity, &mut GlobalTransform, &Speed, &mut Delay), (With<FlameSpirit>, Without<Player>)>, 
    player_query: Query<&GlobalTransform, (With<Player>, Without<FlameSpirit>)>) {
    for (mut velocity, flame_pos, speed, mut delay) in query.iter_mut() {
        for player_pos in player_query.iter() {

            let direction = sprite_movement(flame_pos.translation, player_pos.translation);

            if delay.next_action_aviable(Instant::now()) {
                *velocity = Velocity::from_linear(direction * speed.value);
                delay.start = Instant::now();
            }
        }
    }
}

pub fn poison_entities(mut query: Query<(&mut Health, &mut PoisonDelay, Entity)>) {
    for (mut health, mut poison, _) in query.iter_mut() {
        if !poison.finished() {
            if poison.tick_poison(Instant::now()) {
                health.value -= 1;
            }
        }
    }
}
