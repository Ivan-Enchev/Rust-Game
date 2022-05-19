use crate::structs::*;
use bevy::prelude::*;
use bevy_retrograde::prelude::*;
use crate::Status::*;
use crate::Element::*;
use crate::Specialty::*;
use crate::Direction::*;

pub fn second_ability(keyboard_input: Res<Input<KeyCode>>, mut pos_query: Query<(&GlobalTransform, &mut CurrentStatus, &mut Health, &mut Speed), With<Player>>,
special_delay: Query<&mut Delay, With<Special2>>, inventory: Query<&mut PlayerInventory>, 
mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {
    for (position, mut status, mut health, mut speed) in pos_query.iter_mut() {
        let dark_dark = asset_server.load("dark_dark.png");
        let dark_nature = asset_server.load("dark_nature.png");
        let dark_fire = asset_server.load("dark_fire.png");
        let dark_water = asset_server.load("dark_water.png");
        let dark_air = asset_server.load("dark_air.png");
        let nature_dark = asset_server.load("nature_dark.png");
        let nature_nature = asset_server.load("nature_nature.png");
        let nature_air = asset_server.load("nature_air.png");
        let nature_fire = asset_server.load("nature_fire.png");
        let air_dark = asset_server.load("air_dark.png");
        let air_air = asset_server.load("air_air.png");
        let air_fire = asset_server.load("air_fire.png");
        let water_dark = asset_server.load("water_dark.png");
        let water_air = asset_server.load("water_air.png");
        let fire_dark = asset_server.load("fire_dark.png");
        let fire_nature = asset_server.load("fire_nature.png");
        let fire_air = asset_server.load("fire_air.png");
        let fire_fire = asset_server.load("fire_fire.png");

        let mut attack_direction_x = 0.0;
        let mut attack_direction_y = 0.0;

        inventory.for_each_mut(|mut inventory| {
            special_delay.for_each_mut(|mut delay|{if delay.timer.tick(time.delta()).just_finished(){inventory.can_attack = true;}});
            if inventory.can_attack {
                if keyboard_input.just_pressed(KeyCode::C) {
                    match inventory.facing {
                        Right => attack_direction_x = 30.,
                        Left => attack_direction_x = -30.,
                        Up => attack_direction_y = -30.,
                        Down => attack_direction_y = 30.
                    }
                    
                    match inventory.p_element {
                        Darkness => {
                            match inventory.weapons[inventory.active_weapon as usize] {
                                Darkness => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(20.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: dark_dark.clone(),
                                            transform: Transform::from_xyz(position.translation.x,position.translation.y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: dark_dark.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 5.})
                                        .insert(AttackSpecialty {value: SPNone})
                                        .insert(Delay {timer: Timer::from_seconds(0.5, true)})
                                        .insert_bundle(AttackBundle::default());
                                }
                                Nature => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(30.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: dark_nature.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: dark_nature.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 1.})
                                        .insert(AttackSpecialty {value: Death})
                                        .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                        .insert_bundle(AttackBundle::default());
                                },
                                Air => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(20.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: dark_air.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: dark_air.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 1.})
                                        .insert(AttackSpecialty {value: Half})
                                        .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                        .insert_bundle(AttackBundle::default());
                                },
                                Water => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(15.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: dark_water.clone(),
                                            transform: Transform::from_xyz(position.translation.x,position.translation.y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: dark_water.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 2.})
                                        .insert(AttackSpecialty {value: SPNone})
                                        .insert(Delay {timer: Timer::from_seconds(3., true)})
                                        .insert_bundle(AttackBundle::default());
                                },
                                Fire => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(15.)});
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(30.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: dark_fire.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x * 1.5,
                                                    position.translation.y + attack_direction_y * 1.5, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: dark_fire.clone(),
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
                                ENone => print!("No special weapon.")
                            }
                        },
                        Nature => {
                            match inventory.weapons[inventory.active_weapon as usize] {
                                Darkness => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(20.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: nature_dark.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: nature_dark.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 1.})
                                        .insert(AttackSpecialty {value: StrongPoison})
                                        .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                        .insert_bundle(AttackBundle::default());
                                }
                                Nature => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(12.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: nature_nature.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: nature_nature.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 1.})
                                        .insert(AttackSpecialty {value: LongPoison})
                                        .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                        .insert_bundle(AttackBundle::default());
                                },
                                Air => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(15.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: nature_air.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: nature_air.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 1.})
                                        .insert(AttackSpecialty {value: SlowPoison})
                                        .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                        .insert_bundle(AttackBundle::default());
                                },
                                Water => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(20.)});
                                    status.value = PoisonHeal;
                                },
                                Fire => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(15.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: nature_fire.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: nature_fire.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 1.})
                                        .insert(AttackSpecialty {value: FastPoison})
                                        .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                        .insert_bundle(AttackBundle::default());
                                },
                                ENone => print!("No special weapon.")
                            }
                        },
                        Air => {
                            match inventory.weapons[inventory.active_weapon as usize] {
                                Darkness => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(20.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: air_dark.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: air_dark.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 2.})
                                        .insert(AttackSpecialty {value: SuperWeaken})
                                        .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                        .insert_bundle(AttackBundle::default());
                                }
                                Nature => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(15.)});
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(15.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: nature_air.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: nature_air.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 1.})
                                        .insert(AttackSpecialty {value: SlowPoison})
                                        .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                        .insert_bundle(AttackBundle::default());
                                },
                                Air => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(30.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: air_air.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: air_air.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 1.})
                                        .insert(AttackSpecialty {value: Paralize})
                                        .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                        .insert_bundle(AttackBundle::default());
                                },
                                Water => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(15.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: water_air.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: water_air.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 1.})
                                        .insert(AttackSpecialty {value: Slow})
                                        .insert(Delay {timer: Timer::from_seconds(3., true)})
                                        .insert_bundle(AttackBundle::default());
                                },
                                Fire => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(20.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: air_fire.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: air_fire.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 1.})
                                        .insert(AttackSpecialty {value: Slow})
                                        .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                        .insert_bundle(AttackBundle::default());
                                },
                                ENone => print!("No special weapon.")
                            }
                        },
                        Water => {
                            match inventory.weapons[inventory.active_weapon as usize] {
                                Darkness => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(30.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: water_dark.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: water_dark.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 10.})
                                        .insert(AttackSpecialty {value: SPNone})
                                        .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                        .insert_bundle(AttackBundle::default());
                                }
                                Nature => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(25.)});
                                    if health.value < inventory.max_health {
                                        health.value += 1;
                                    }
                                },
                                Air => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(15.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: water_air.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: air_fire.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 1.})
                                        .insert(AttackSpecialty {value: Slow})
                                        .insert(Delay {timer: Timer::from_seconds(3., true)})
                                        .insert_bundle(AttackBundle::default());
                                },
                                Water => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(20.)});
                                    commands
                                        .spawn()
                                        .insert(ProtectionDelay)
                                        .insert(Delay {timer: Timer::from_seconds(10., true)});
                                    status.value = Protected;
                                    speed.value = 100.;
                                },
                                Fire => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(20.)});
                                    commands
                                        .spawn()
                                        .insert(ProtectionDelay)
                                        .insert(Delay {timer: Timer::from_seconds(10., true)});
                                    speed.value = 120.;
                                },
                                ENone => print!("No special weapon.")
                            }
                        },
                        Fire => {
                            match inventory.weapons[inventory.active_weapon as usize] {
                                Darkness => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(15.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: fire_dark.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x * 2.
                                                ,position.translation.y + attack_direction_y * 2., 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: fire_dark.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 2.})
                                        .insert(AttackSpecialty {value: SPNone})
                                        .insert(Delay {timer: Timer::from_seconds(0.5, true)})
                                        .insert_bundle(AttackBundle::default());
                                }
                                Nature => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(10.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: fire_nature.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: fire_nature.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 1.})
                                        .insert(AttackSpecialty {value: ShortPoison})
                                        .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                        .insert_bundle(AttackBundle::default());
                                },
                                Air => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(10.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: fire_air.clone(),
                                            transform: Transform::from_xyz(position.translation.x + attack_direction_x,
                                                    position.translation.y + attack_direction_y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: fire_air.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 2.})
                                        .insert(AttackSpecialty {value: LowWeaken})
                                        .insert(Delay {timer: Timer::from_seconds(0.2, true)})
                                        .insert_bundle(AttackBundle::default());
                                },
                                Water => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(25.)});
                                    commands
                                        .spawn()
                                        .insert(ProtectionDelay)
                                        .insert(Delay {timer: Timer::from_seconds(5., true)});
                                    speed.value = 150.;
                                },
                                Fire => {
                                    special_delay.for_each_mut(|mut delay| {delay.change_timer(10.)});
                                    commands
                                        .spawn_bundle(SpriteBundle {
                                            image: fire_fire.clone(),
                                            transform: Transform::from_xyz(position.translation.x,position.translation.y, 0.),
                                            ..Default::default()
                                        })
                                        .insert(TesselatedCollider {
                                            image: fire_fire.clone(),
                                            tesselator_config: TesselatedColliderConfig {
                                                vertice_separation: 0.,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Damage{value: 2.})
                                        .insert(AttackSpecialty {value: SPNone})
                                        .insert(Delay {timer: Timer::from_seconds(0.5, true)})
                                        .insert_bundle(AttackBundle::default());
                                },
                                ENone => print!("No special weapon.")
                            }
                        },
                        ENone => print!("Player doesn't have element!")
                    }
                    inventory.can_attack = false;
                    special_delay.for_each_mut(|mut delay|{delay.timer.reset()});
                }
            }
        });
    }
}