//use std::ops::RangeBounds;

use bevy::prelude::*;
use bevy_retrograde::prelude::*;
use rand::{thread_rng, Rng};


fn main() {
    App::build()
        .add_plugins(RetroPlugins)
        .add_startup_system(setup.system())
    	.add_system(move_player.system())
        .add_system(detect_collisions.system())
        .add_system(move_camera.system())
        .run();
}

#[derive(PhysicsLayer)]
enum Layer {
    Enemy,
    Player,
}
struct Player {
    speed: f32
}
//struct Block;
//struct Spike;
struct Health {
    value: i8,
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
                //.insert(Block);
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
                //.insert(Block);
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
            //.insert(Block);
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
                //.insert(Block);
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
            //.insert(Block);
        }
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
                // We want the collision shape for the player to be highly accurate
                vertice_separation: 0.,
                ..Default::default()
            },
            ..Default::default()
        })
        // The player is also a dynamic body with rotations locked
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        // Disable friction and bounciness
        .insert(PhysicMaterial {
            friction: 0.,
            restitution: 0.,
            ..Default::default()
        })
        // Set the player speed to 0 initially
        .insert(Velocity::from_linear(Vec3::default()))
        .insert(Player {speed: 75.})
        .insert(Health {value: 10})
        .insert(CollisionLayers::new(Layer::Player, Layer::Enemy));
        
    
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>, player_q: Query<&Player>,
    mut cam_query: Query<&mut Transform, With<Camera>>)  {
    for mut velocity in query.iter_mut() {

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

        for player in player_q.iter() {
            *velocity = Velocity::from_linear(direction * player.speed);       
            for mut transform in cam_query.iter_mut() {
                transform.translation += direction * player.speed * 0.01 * 1.5;
            }
        }

    }
}

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in query.iter_mut() {
        const SPEED: f32 = 1.;

        let mut direction = Vec3::new(0., 0., 0.);

        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-SPEED, 0., 0.);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(SPEED, 0., 0.);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0., -SPEED, 0.);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0., SPEED, 0.);
        }

        direction = direction.normalize_or_zero();

        transform.translation += direction;
    }
}

fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Player) && !layers.contains_group(Layer::Enemy)
}

fn is_enemy(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::Player) && layers.contains_group(Layer::Enemy)
}

fn detect_collisions(mut events: EventReader<CollisionEvent>, mut health_query: Query<&mut Health, With<Player>>) {

    for event in events.iter().filter(|e| e.is_started()) {    
        for mut health in health_query.iter_mut() {
            let (layers_1, layers_2) = event.collision_layers();
            if is_player(layers_1) && is_enemy(layers_2) {
                health.value -= 1;
                println!("Health {}", health.value);
            } else if is_player(layers_2) && is_enemy(layers_1) {
                health.value -= 1;
                println!("Health {}", health.value);
            }   
        }
    }                
}
