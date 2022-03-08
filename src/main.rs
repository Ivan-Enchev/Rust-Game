mod structs;
mod player_mechanics;
mod enemy_mechanics;
mod collisions;
mod spawn_entities;
mod menus;

use bevy::prelude::*;
use bevy_retrograde::prelude::*;
use crate::structs::*;
use crate::player_mechanics::*;
use crate::enemy_mechanics::*;
use crate::collisions::*;
use crate::spawn_entities::*;
use crate::menus::*;
use crate::Element::ENone;
use rand::{thread_rng, Rng};

fn main() {
    App::build()
        .add_plugins(RetroPlugins)
        .add_state(GameState::Menu)
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(menu.system())
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Menu)
                .with_system(menu_cleanup.system())
        )
        .add_system_set(
            SystemSet::on_enter(GameState::ElementSelect)
                .with_system(spawn_elements.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::ElementSelect)
                .with_system(element_select.system())
        )
        .add_system_set(
            SystemSet::on_exit(GameState::ElementSelect)
                .with_system(element_cleanup.system())
        )
        .add_system_set(
            SystemSet::on_enter(GameState::LevelSelection)
                .with_system(spawn_rooms.system())
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Heal)
                .with_system(heal.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::LevelSelection)
                .with_system(level_select.system())
        )
        .add_system_set(
            SystemSet::on_exit(GameState::LevelSelection)
                .with_system(level_select_cleanup.system())
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Started)
                .with_system(spawn_enemies.system())
                .with_system(spawn_blocks.system())
                .with_system(spawn_player.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::Started)
                .with_system(move_player.system())
                .with_system(detect_collisions.system())
                .with_system(detect_enemy_collisions.system())
                .with_system(move_slime.system())
                .with_system(move_flame_spirit.system())
                .with_system(player_attack.system())
                .with_system(special_attack.system())
                .with_system(poison_entities.system())
                .with_system(remove_protection.system())
                .with_system(end_attack.system())
                .with_system(despawn_defeated.system())
                .with_system(level_end_system.system())
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Started)
                .with_system(level_cleanup.system())
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let start_button = asset_server.load("start_button.png");
    let quit_button = asset_server.load("quit_button.png");
    
    commands.spawn_bundle(CameraBundle {
        camera: Camera {
            size: CameraSize::FixedHeight(200),
            background_color: Color::new(1.0, 1.0, 1.0, 1.0),
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
    let mut room2 = [0; 5];
    let mut room3 = [0; 5];
    let mut rng = thread_rng();
        let mut number: i16 = rng.gen_range(0..90);
        for i in 0..5 {
            let room;
            match number {
                0..=59 => room = 1,
                60..=69 => room = 2,
                70..=79 => room = 3,
                80..=89 => room = 4,
                _ => room = 0
            }
            room2[i] = room;
            number = rng.gen_range(0..90);
        }

        for i in 0..5 {
            let room;
            match number {
                0..=59 => room = 1,
                60..=69 => room = 2,
                70..=79 => room = 3,
                80..=89 => room = 4,
                _ => room = 0
            }
            room3[i] = room;
            number = rng.gen_range(0..90);
        }


    commands
        .spawn()
        .insert(GameStage{
            level: 1,
            rooms_1: [0, 1, 1, 1, 0],
            active_room: 0,
            start_point: 2,
            rooms_2: room2,
            rooms_3: room3,
            arrow_pos: 20.,
            enemies: 0
        })
        .insert(PlayerInventory {weapon_1: ENone, weapon_2: ENone, active_weapon: 0, p_health: 10,
            coins: 0, p_element: ENone, can_attack: false}
        );

    commands
        .spawn()
        .insert(KeyDelay)
        .insert(Delay {timer: Timer::from_seconds(0.5, false)});

    commands
        .spawn()
        .insert(Special1)
        .insert(Delay {timer: Timer::from_seconds(1., false)});

    commands
        .spawn_bundle(SpriteBundle {
            image: start_button.clone(),
            transform: Transform::from_xyz(0., -20., 0.),
            ..Default::default()
        })
        .insert(Button {is_active: true, id: 1});
    
        commands
        .spawn_bundle(SpriteBundle {
            image: quit_button.clone(),
            transform: Transform::from_xyz(0., 30., 0.),
            ..Default::default()
        })
        .insert(Button {is_active: false, id: 2});
    
}


