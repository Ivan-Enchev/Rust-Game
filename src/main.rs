use bevy::prelude::*;
use bevy_retrograde::prelude::*;
use rand::{thread_rng, Rng};
use std::{time::Duration, thread::sleep};
use std::time::Instant; 

fn main() {
    App::build()
        .add_plugins(RetroPlugins)
        .add_startup_system(setup.system())
    	.add_system(move_player.system())
        .add_system(detect_collisions.system())
        .add_system(detect_slime_collisions.system())
        .add_system(detect_flame_collisions.system())
        .add_system(move_slime.system())
        .add_system(move_flame_spirit.system())
        .add_system(player_attack.system())
        .add_system(end_attack.system())
        .add_system(despawn_defeated.system())
        .run();
}

#[derive(PhysicsLayer)]
enum Layer {
    Enemy,
    Player,
    Projectile,
}

struct FlameSpirit;
struct Slime;
struct Player;
struct BasicAttack;
struct Speed {
    value: f32
}
struct Health {
    value: i8,
}

struct Delay {
    start: Instant,
    delay: f64,
}

impl Delay {
    fn next_action_aviable(&self, now: Instant) -> bool {
        now.duration_since(self.start).as_secs_f64() >= self.delay
    }
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

fn despawn_defeated(mut commands: Commands, query: Query<(&Health, Entity)>) {
    for (health, entity) in query.iter() {
        if health.value <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &GlobalTransform, &Speed), With<Player>>,
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

fn player_attack(keyboard_input: Res<Input<KeyCode>>, pos_query: Query<&GlobalTransform, With<Player>>,
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
                .insert(CollisionLayers::new(Layer::Projectile, Layer::Enemy));
        }
    }
}

fn end_attack(mut commands: Commands, query: Query<(&BasicAttack, Entity, &Delay)>) {
    for (_, entity, delay) in query.iter() {
        if delay.next_action_aviable(Instant::now()) {
            commands.entity(entity).despawn();
        }
    }
}


fn move_slime(mut query: Query<(&mut Velocity, &mut GlobalTransform, &Speed), (With<Slime>, Without<Player>)>, 
    player_query: Query<&GlobalTransform, (With<Player>, Without<Slime>)>) {
    for (mut velocity, slime_pos, speed) in query.iter_mut() {
        for player_pos in player_query.iter() {
            let mut direction = Vec3::new(0., 0., 0.);

            if slime_pos.translation.x > player_pos.translation.x {
                direction += Vec3::new(-1.0, 0., 0.);
            }
    
            if slime_pos.translation.x < player_pos.translation.x {
                direction += Vec3::new(1.0, 0., 0.);
            }
    
            if slime_pos.translation.y > player_pos.translation.y {
                direction += Vec3::new(0., -1.0, 0.);
            }
    
            if slime_pos.translation.y < player_pos.translation.y {
                direction += Vec3::new(0., 1.0, 0.);
            }

            direction = direction.normalize_or_zero();

            *velocity = Velocity::from_linear(direction * speed.value);
        }

    }
}

fn move_flame_spirit(mut query: Query<(&mut Velocity, &mut GlobalTransform, &Speed, &mut Delay), (With<FlameSpirit>, Without<Player>)>, 
    player_query: Query<&GlobalTransform, (With<Player>, Without<FlameSpirit>)>) {
    for (mut velocity, flame_pos, speed, mut delay) in query.iter_mut() {
        for player_pos in player_query.iter() {
            let mut direction = Vec3::new(0., 0., 0.);

            if flame_pos.translation.x > player_pos.translation.x {
                direction += Vec3::new(-1.0, 0., 0.);
            }
    
            if flame_pos.translation.x < player_pos.translation.x {
                direction += Vec3::new(1.0, 0., 0.);
            }
    
            if flame_pos.translation.y > player_pos.translation.y {
                direction += Vec3::new(0., -1.0, 0.);
            }
    
            if flame_pos.translation.y < player_pos.translation.y {
                direction += Vec3::new(0., 1.0, 0.);
            }

            direction = direction.normalize_or_zero();

            if delay.next_action_aviable(Instant::now()) {
                *velocity = Velocity::from_linear(direction * speed.value);
                delay.start = Instant::now();
            }
        }



    }
}

fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Player) && !layers.contains_group(Layer::Enemy)
}

fn is_enemy(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::Player) && layers.contains_group(Layer::Enemy)
}

fn is_projectile(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Projectile) && !layers.contains_group(Layer::Enemy)
}

fn detect_collisions(mut events: EventReader<CollisionEvent>, mut player_health_query: Query<&mut Health, With<Player>>) {

    for event in events.iter().filter(|e| e.is_started()) {    
        let mut health = if let Some(health) = player_health_query.iter_mut().next() { health } else { return; };
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

fn detect_slime_collisions(mut events: EventReader<CollisionEvent>, mut enemy_health_query: Query<&mut Health, (With<Slime>, Without<FlameSpirit>)>) {

    for event in events.iter().filter(|e| e.is_started()) {    
        let mut health = if let Some(health) = enemy_health_query.iter_mut().next() { health } else { return; };
        let (layers_1, layers_2) = event.collision_layers();
        if is_projectile(layers_1) && is_enemy(layers_2) {
            health.value -= 1;
            println!("Enemy Health {}", health.value);
        } else if is_projectile(layers_2) && is_enemy(layers_1) {
            health.value -= 1;
            println!("Enemy Health {}", health.value);
        }   
    }  
}

fn detect_flame_collisions(mut events: EventReader<CollisionEvent>, mut enemy_health_query: Query<&mut Health, (With<FlameSpirit>, Without<Slime>)>) {

    for event in events.iter().filter(|e| e.is_started()) {    
        let mut health = if let Some(health) = enemy_health_query.iter_mut().next() { health } else { return; };
        let (layers_1, layers_2) = event.collision_layers();
        if is_projectile(layers_1) && is_enemy(layers_2) {
            health.value -= 1;
            println!("Enemy Health {}", health.value);
        } else if is_projectile(layers_2) && is_enemy(layers_1) {
            health.value -= 1;
            println!("Enemy Health {}", health.value);
        }   
    }  
}

