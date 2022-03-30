use crate::structs::*;
use bevy::prelude::*;
use bevy_retrograde::prelude::*;
use crate::Status::*;
use crate::Specialty::*;

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
                .insert(Damage{value: 2.})
                .insert(AttackSpecialty {value: SPNone})
                .insert(Delay {timer: Timer::from_seconds(0.1, true)})
                .insert_bundle(AttackBundle::default());
        }
    }
}

pub fn end_attack(mut commands: Commands, mut query: Query<(&BasicAttack, Entity, &mut Delay)>, time: Res<Time>) {
    for (_, entity, mut delay) in query.iter_mut() {
        if delay.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn special_attack(keyboard_input: Res<Input<KeyCode>>, mut pos_query: Query<(&GlobalTransform, &mut CurrentStatus, &mut Velocity), With<Player>>,
special_delay: Query<&mut Delay, With<Special1>>, inventory: Query<&mut PlayerInventory>, 
mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {
    for (position, mut status, mut velocity) in pos_query.iter_mut() {
        let dark_attack = asset_server.load("dark_attack.png");
        let nature_attack = asset_server.load("nature_attack.png");
        let air_attack = asset_server.load("air_attack.png");
        let mut attack_direction_x = 0.0;
        let mut attack_direction_y = 0.0;

        if keyboard_input.pressed(KeyCode::Right) {
            attack_direction_x = 30.;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            attack_direction_x = -30.;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            attack_direction_y = -30.;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            attack_direction_y = 30.;
        }

        inventory.for_each_mut(|mut inventory| {
            special_delay.for_each_mut(|mut delay|{if delay.timer.tick(time.delta()).just_finished(){inventory.can_attack = true;}});
            if inventory.can_attack {
                if keyboard_input.just_pressed(KeyCode::X) {
                    if attack_direction_x == 0. && attack_direction_y == 0. {
                        attack_direction_x = 10.;
                    }
                    if attack_direction_x == 0. && attack_direction_y == 0. {
                        attack_direction_x = 30.;
                    }
                    
                    match inventory.p_element {
                        Element::Darkness => {
                            commands
                                .spawn_bundle(SpriteBundle {
                                    image: dark_attack.clone(),
                                    transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                            position.translation.y + attack_direction_y, 0.),
                                    ..Default::default()
                                })
                                .insert(TesselatedCollider {
                                    image: dark_attack.clone(),
                                    tesselator_config: TesselatedColliderConfig {
                                        vertice_separation: 0.,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(Damage{value: 4.})
                                .insert(AttackSpecialty {value: SPNone})
                                .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                .insert_bundle(AttackBundle::default());
                        },
                        Element::Nature => {
                            commands
                                .spawn_bundle(SpriteBundle {
                                    image: nature_attack.clone(),
                                    transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                            position.translation.y + attack_direction_y, 0.),
                                    ..Default::default()
                                })
                                .insert(TesselatedCollider {
                                    image: nature_attack.clone(),
                                    tesselator_config: TesselatedColliderConfig {
                                        vertice_separation: 0.,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(Damage{value: 1.})
                                .insert(AttackSpecialty {value: Poison})
                                .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                .insert_bundle(AttackBundle::default());
                        },
                        Element::Fire => {
                            *velocity = Velocity::from_linear(Vec3::new(attack_direction_x * 200., attack_direction_y * 200., 0.));
                        },
                        Element::Water => {
                            commands
                                .spawn()
                                .insert(ProtectionDelay)
                                .insert(Delay {timer: Timer::from_seconds(10., true)});
                            status.value = Protected;
                        },
                        Element::Air => {
                            commands
                                .spawn_bundle(SpriteBundle {
                                    image: air_attack.clone(),
                                    transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                            position.translation.y + attack_direction_y, 0.),
                                    ..Default::default()
                                })
                                .insert(TesselatedCollider {
                                    image: air_attack.clone(),
                                    tesselator_config: TesselatedColliderConfig {
                                        vertice_separation: 0.,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(Damage{value: 2.})
                                .insert(AttackSpecialty {value: Weaken})
                                .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                .insert_bundle(AttackBundle::default());
                        },
                        Element::ENone => print!("No player element!")
                    }
                    inventory.can_attack = false;
                    special_delay.for_each_mut(|mut delay|{delay.timer.reset()});
                }
            }
        });
    }
}


pub fn remove_protection(mut query: Query<(Entity, &mut Delay, &ProtectionDelay)>, player_query: Query<(&mut CurrentStatus, &mut Speed), With<Player>>,
mut commands: Commands, time: Res<Time>) {
    for (entity, mut delay, _) in query.iter_mut() {
        if delay.timer.tick(time.delta()).just_finished() {
            player_query.for_each_mut(|(mut status, mut speed)|{status.value = SNone; speed.value = 75.});
            commands.entity(entity).despawn();
        }
    }
}








