use crate::structs::*;
use bevy::prelude::*;
use bevy_retrograde::prelude::*;
use std::time::Instant; 

pub fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &GlobalTransform, &Speed), With<Player>>,
mut cam_query: Query<&mut Transform, With<Camera>>)  {
    for (mut velocity, position, speed) in query.iter_mut() {

        let mut direction = Vec3::new(0., 0., 0.);

        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0., 0.);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0., 0.);
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0., -1.0, 0.);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0., 1.0, 0.);
        }

        direction = direction.normalize_or_zero();

        *velocity = Velocity::from_linear(direction * speed.value);       
        for mut transform in cam_query.iter_mut() {
            transform.translation = position.translation;
        }
        

    }
}

pub fn player_attack(keyboard_input: Res<Input<KeyCode>>, pos_query: Query<&GlobalTransform, With<Player>>,
mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut sword_txt = "sword.png";
    for position in pos_query.iter() {
        let mut attack_direction_x = 0.0;
        let mut attack_direction_y = 0.0;

        if keyboard_input.pressed(KeyCode::Right) {
            attack_direction_x = 13.;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            attack_direction_x = -13.;
            sword_txt = "sword_r.png";
        }

        if keyboard_input.pressed(KeyCode::Up) {
            attack_direction_y = -15.;
            sword_txt = "sword_u.png";
        }

        if keyboard_input.pressed(KeyCode::Down) {
            attack_direction_y = 15.;
            sword_txt = "sword_d.png";
        }

        let sword = asset_server.load(sword_txt);
        if keyboard_input.just_pressed(KeyCode::Z) {
            if attack_direction_x == 0. && attack_direction_y == 0. {
                attack_direction_x = 10.;
            }
            commands
                .spawn_bundle(SpriteBundle {
                    image: sword.clone(),
                    transform: Transform::from_xyz(position.translation.x + attack_direction_x, position.translation.y + attack_direction_y, 0.),
                    ..Default::default()
                })
                .insert(TesselatedCollider {
                    image: sword.clone(),
                    tesselator_config: TesselatedColliderConfig {
                        vertice_separation: 0.,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(RigidBody::Sensor)
                .insert(Velocity::from_linear(Vec3::default()))
                .insert(BasicAttack)
                .insert(Delay {delay: 0.1, start: Instant::now()})
                .insert(CollisionLayers::new(Layer::Projectile, Layer::Enemy))
                .insert(LevelEntity);
        }
    }
}

pub fn end_attack(mut commands: Commands, query: Query<(&BasicAttack, Entity, &Delay)>) {
    for (_, entity, delay) in query.iter() {
        if delay.next_action_aviable(Instant::now()) {
            commands.entity(entity).despawn();
        }
    }
}
