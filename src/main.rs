mod structs;
mod player_mechanics;
mod enemy_mechanics;
mod collisions;

use bevy::prelude::*;
use bevy_retrograde::prelude::*;
use rand::{thread_rng, Rng};
use std::time::Instant; 
use crate::structs::*;
use crate::player_mechanics::*;
use crate::enemy_mechanics::*;
use crate::collisions::*;

fn main() {
    App::build()
        .add_plugins(RetroPlugins)
        .add_startup_system(setup.system())
    	.add_system(move_player.system())
        .add_system(detect_collisions.system())
        .add_system(detect_enemy_collisions.system())
        .add_system(move_slime.system())
        .add_system(move_flame_spirit.system())
        .add_system(player_attack.system())
        .add_system(end_attack.system())
        .add_system(despawn_defeated.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(CameraBundle {
        camera: Camera {
            size: CameraSize::FixedHeight(200),
            background_color: Color::new(1.0, 1.0, 1.0, 1.0),
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });

    let player = asset_server.load("player.png");
    let block = asset_server.load("block.png");
    let spike = asset_server.load("spike.png");
    let slime = asset_server.load("slime.png");
    let flame = asset_server.load("flame.png");

    let mut rng = thread_rng();

    let mut number: i16 = rng.gen_range(0..20);

    let map_max = 400.0;
    let map_min = -400.0;

    while number < 20 {
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
                .insert(RigidBody::Static);
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
                .insert(RigidBody::Static);
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
            .insert(RigidBody::Static);
        }
        number += 1;
    }

    number = rng.gen_range(0..30);

    while number < 30 {
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
            .insert(CollisionLayers::new(Layer::Enemy, Layer::Player));
        }
        number += 1;
    }

    number = rng.gen_range(1..10);

    while number < 10 {
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
            .insert(Enemy)
            .insert(Speed {value: 20.})
            .insert(Health {value: 3})
            .insert(
                CollisionLayers::none()
                    .with_group(Layer::Enemy)
                    .with_masks(&[Layer::Player, Layer::Projectile])
            );
        number += 1;
    }

    number = rng.gen_range(0..6);

    while number < 6 {
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
            .insert(Delay {start: Instant::now(), delay: 2.})
            .insert(Speed {value: 60.})
            .insert(Health {value: 2})
            .insert(
                CollisionLayers::none()
                    .with_group(Layer::Enemy)
                    .with_masks(&[Layer::Player, Layer::Projectile])
            );
        number += 1;
    }
    
    
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
        .insert(Speed {value: 75.})
        .insert(Health {value: 10})
        .insert(CollisionLayers::new(Layer::Player, Layer::Enemy));  
    
}





