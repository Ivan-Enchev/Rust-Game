use bevy::{prelude::*, ecs::system::QuerySingleError};
use bevy_retrograde::prelude::*;
use rand::{thread_rng, Rng};
use std::time::Instant; 
use crate::structs::*;

pub fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>, stage_query: Query<&mut GameStage>) {
    let slime = asset_server.load("slime.png");
    let flame = asset_server.load("flame.png");
    let mut current_level = 0;
    let mut elite_multiplier = 1;
    let mut enemy_number = 0;
    stage_query.for_each_mut(|stage| {current_level = stage.level; 
        if stage.rooms_1[stage.active_room as usize] == 4 {
            elite_multiplier = 2;
        }
        enemy_number = stage.enemies;
    });

    let mut rng = thread_rng();

    let mut number: i16 = rng.gen_range(0..=(current_level + 3));

    let map_max = 200.0 + (100. * (current_level as f32 / 10.));
    let map_min = -200.0 - (100. * (current_level as f32 / 10.));

    let slime_speed = (20. + (current_level as f32 * 0.5))* elite_multiplier as f32;
    let slime_health = (3 + current_level / 10) * elite_multiplier;

    let flame_health = (2 + current_level / 10) * elite_multiplier;

    
    while number <= current_level + 3 {
        let mut x: f32 = rng.gen_range(map_min..map_max);
        let mut y: f32 = rng.gen_range(map_min..map_max);
        while (-20. < x && x < 20.) || (-20. < y && y < 20.) {
            x = rng.gen_range(map_min..map_max);
            y = rng.gen_range(map_min..map_max);
        }

        commands
            .spawn_bundle(SpriteBundle {
                image: slime.clone(),
                transform: Transform::from_xyz(x, y, 0.),
                ..Default::default()
            })
            .insert(TesselatedCollider {
                image: slime.clone(),
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            .insert(RotationConstraints::lock())
            .insert(Velocity::from_linear(Vec3::default()))
            .insert(Slime)
            .insert(LevelEntity)
            .insert(Enemy)
            .insert(Speed {value: slime_speed})
            .insert(Health {value: slime_health})
            .insert(
                CollisionLayers::none()
                    .with_group(Layer::Enemy)
                    .with_masks(&[Layer::Player, Layer::Projectile])
            );
        number += 1;
        enemy_number += 1;
    }

    if current_level >= 3 {
        number = rng.gen_range(0..=(current_level - 2));
    }

    while number <= current_level - 2 && current_level >= 3 {
        let mut x: f32 = rng.gen_range(map_min..map_max);
        let mut y: f32 = rng.gen_range(map_min..map_max);
        while (-20. < x && x < 20.) || (-20. < y && y < 20.) {
            x = rng.gen_range(map_min..map_max);
            y = rng.gen_range(map_min..map_max);
        }

        commands
            .spawn_bundle(SpriteBundle {
                image: flame.clone(),
                transform: Transform::from_xyz(x, y, 0.),
                ..Default::default()
            })
            .insert(TesselatedCollider {
                image: flame.clone(),
                tesselator_config: TesselatedColliderConfig {
                    vertice_separation: 0.,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            .insert(RotationConstraints::lock())
            .insert(Velocity::from_linear(Vec3::default()))
            .insert(FlameSpirit)
            .insert(Enemy)
            .insert(LevelEntity)
            .insert(Delay {start: Instant::now(), delay: 2.})
            .insert(Speed {value: 60.})
            .insert(Health {value: flame_health})
            .insert(
                CollisionLayers::none()
                    .with_group(Layer::Enemy)
                    .with_masks(&[Layer::Player, Layer::Projectile])
            );
            enemy_number += 1;
        number += 1;
    }

    stage_query.for_each_mut(|mut stage| {stage.enemies = enemy_number});

}

pub fn spawn_blocks(mut commands: Commands, asset_server: Res<AssetServer>, stage_query: Query<&GameStage>) {
    let block = asset_server.load("block.png");
    let spike = asset_server.load("spike.png");
    let mut current_level = 0;
    stage_query.for_each(|stage| {current_level = stage.level});

    let mut rng = thread_rng();

    let mut number: i16 = rng.gen_range(0..=(current_level + 9));

    let map_max = 200.0 + (100. * (current_level as f32 / 10.));
    let map_min = -200.0 - (100. * (current_level as f32 / 10.));

    while number <= current_level + 5 {
        let mut x: f32 = rng.gen_range(map_min..map_max);
        let mut y: f32 = rng.gen_range(map_min..map_max);
        while (-20. < x && x < 20.) || (-20. < y && y < 20.) {
            x = rng.gen_range(map_min..map_max);
            y = rng.gen_range(map_min..map_max);
        }

        if number % 7 == 0 {            
            for _i in 0..5 {
                commands
                .spawn_bundle(SpriteBundle {
                    image: block.clone(),
                    transform: Transform::from_xyz(x, y, 0.),
                    ..Default::default()
                })
                .insert(TesselatedCollider {
                    image: block.clone(),
                    ..Default::default()
                })
                .insert(RigidBody::Static)
                .insert(LevelEntity);
                x += 12.;
            }
        }
        else if number % 3 == 0{
            for _i in 0..2 {
                commands
                .spawn_bundle(SpriteBundle {
                    image: block.clone(),
                    transform: Transform::from_xyz(x, y, 0.),
                    ..Default::default()
                })
                .insert(TesselatedCollider {
                    image: block.clone(),
                    ..Default::default()
                })
                .insert(RigidBody::Static)
                .insert(LevelEntity);
                if x < 0. {
                    x+=12.;
                    y+=12.;
                }
                else {
                    x-=12.;
                    y+=12.;
                }
            }
        }
        else {
            commands
            .spawn_bundle(SpriteBundle {
                image: block.clone(),
                transform: Transform::from_xyz(x, y, 0.),
                ..Default::default()
            })
            .insert(TesselatedCollider {
                image: block.clone(),
                ..Default::default()
            })
            .insert(RigidBody::Static)
            .insert(LevelEntity);
        }
        number += 1;
    }

    number = rng.gen_range(0..=(current_level + 3));

    while number <= current_level + 5 {
        let mut x: f32 = rng.gen_range(map_min..map_max);
        let mut y: f32 = rng.gen_range(map_min..map_max);
        while (-20. < x && x < 20.) || (-20. < y && y < 20.) {
            x = rng.gen_range(map_min..map_max);
            y = rng.gen_range(map_min..map_max);
        }

        if number % 7 == 0 {            
            for _i in 0..3 {
                commands
                .spawn_bundle(SpriteBundle {
                    image: spike.clone(),
                    transform: Transform::from_xyz(x, y, 0.),
                    ..Default::default()
                })
                .insert(TesselatedCollider {
                    image: spike.clone(),
                    ..Default::default()
                })
                .insert(RigidBody::Static)
                .insert(LevelEntity)
                .insert(CollisionLayers::new(Layer::Enemy, Layer::Player));
                y -= 12.;
            }
        }
        else {
            commands
            .spawn_bundle(SpriteBundle {
                image: spike.clone(),
                transform: Transform::from_xyz(x, y, 0.),
                ..Default::default()
            })
            .insert(TesselatedCollider {
                image: spike.clone(),
                ..Default::default()
            })
            .insert(RigidBody::Static)
            .insert(LevelEntity)
            .insert(CollisionLayers::new(Layer::Enemy, Layer::Player));
        }
        number += 1;
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, inventory: Query<&PlayerInventory>, ) {
    
    let player = asset_server.load("player.png");

    let mut current_health = 0;

    inventory.for_each(|inventory| {current_health = inventory.p_health});

    commands
    .spawn_bundle(SpriteBundle {
        image: player.clone(),
        sprite: Sprite {
            pixel_perfect: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    })
    .insert(TesselatedCollider {
        image: player.clone(),
        tesselator_config: TesselatedColliderConfig {
            vertice_separation: 0.,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(RigidBody::Dynamic)
    .insert(RotationConstraints::lock())
    .insert(Velocity::from_linear(Vec3::default()))
    .insert(Player)
    .insert(LevelEntity)
    .insert(Speed {value: 75.})
    .insert(Health {value: current_health})
    .insert(CollisionLayers::new(Layer::Player, Layer::Enemy));  

    
}

pub fn spawn_rooms(mut commands: Commands, asset_server: Res<AssetServer>, stage_query: Query<&GameStage>) {
    match stage_query.single() {
        Ok(&GameStage { level: _, rooms_1, rooms_2, rooms_3, active_room: _ ,
            arrow_pos, start_point: _, enemies:_}) => {
            let mut room_icon;
            for i in 0..5 {
                match rooms_1[i] {
                    1 => room_icon = asset_server.load("monster_room.png"),
                    2 => room_icon = asset_server.load("shop_room.png"),
                    3 => room_icon = asset_server.load("safe_room.png"),
                    4 => room_icon = asset_server.load("elite_room.png"),
                    5 => room_icon = asset_server.load("artifact_room.png"),
                    _ => continue,
                }
                commands
                    .spawn_bundle(SpriteBundle {
                        image: room_icon.clone(),
                        transform: Transform::from_xyz(-40. + 30. * i as f32, -40., 0.),
                        ..Default::default()
                    })
                    .insert(Room);
            }

            for i in 0..5 {
                match rooms_2[i] {
                    1 => room_icon = asset_server.load("monster_room.png"),
                    2 => room_icon = asset_server.load("shop_room.png"),
                    3 => room_icon = asset_server.load("safe_room.png"),
                    4 => room_icon = asset_server.load("elite_room.png"),
                    5 => room_icon = asset_server.load("artifact_room.png"),
                    _ => continue,
                }
                commands
                    .spawn_bundle(SpriteBundle {
                        image: room_icon.clone(),
                        transform: Transform::from_xyz(-40. + 30. * i as f32, 10., 0.),
                        ..Default::default()
                    })
                    .insert(Room);
            }

            for i in 0..5 {
                match rooms_3[i] {
                    1 => room_icon = asset_server.load("monster_room.png"),
                    2 => room_icon = asset_server.load("shop_room.png"),
                    3 => room_icon = asset_server.load("safe_room.png"),
                    4 => room_icon = asset_server.load("elite_room.png"),
                    5 => room_icon = asset_server.load("artifact_room.png"),
                    _ => continue,
                }
                commands
                    .spawn_bundle(SpriteBundle {
                        image: room_icon.clone(),
                        transform: Transform::from_xyz(-40. + 30. * i as f32, 60., 0.),
                        ..Default::default()
                    })
                    .insert(Room);
            }
            let choice_arrow = asset_server.load("choice_arrow.png");

            commands
                    .spawn_bundle(SpriteBundle {
                        image: choice_arrow.clone(),
                        transform: Transform::from_xyz(arrow_pos, -65., 0.),
                        ..Default::default()
                    })
                    .insert(ChoiceArrow);
        }
        Err(QuerySingleError::NoEntities(_)) => {
            println!("Error: There is no Game Stage!");
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            println!("Error: There is more than one Game Stage!");
        }
    }
}