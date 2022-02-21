mod structs;
mod player_mechanics;
mod enemy_mechanics;
mod collisions;
mod spawn_entities;

use bevy::prelude::*;
use bevy_retrograde::prelude::*;
use bevy::app::AppExit;
use crate::structs::*;
use crate::player_mechanics::*;
use crate::enemy_mechanics::*;
use crate::collisions::*;
use crate::spawn_entities::*;

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
                .with_system(end_attack.system())
                .with_system(despawn_defeated.system())
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



fn menu(keyboard_input: ResMut<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>, asset_server: Res<AssetServer>, 
mut button_query: Query<(&mut Button, &GlobalTransform), With<Button>>, 
mut commands: Commands, active_query: Query<(Entity, &Active)>, mut exit: EventWriter<AppExit>) {
    let active_button = asset_server.load("active_button.png");

    button_query.for_each_mut(|(button, button_pos)| {
        if button.is_active {
            commands
                .spawn_bundle(SpriteBundle {
                    image: active_button.clone(),
                    transform: Transform::from_xyz(button_pos.translation.x, button_pos.translation.y, 0.),
                    ..Default::default()
                })
                .insert(Active);
        }
    });

    if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::Down) {
        let mut current_button = 0;
        for (mut button, _) in button_query.iter_mut() {
            if button.is_active {
                current_button = button.id + 1;
                if current_button >= 3 {
                    current_button = 1;
                }
                button.is_active = false;
            }
        }
        for (mut button, _) in button_query.iter_mut() {
            if button.id == current_button {
                button.is_active = true;
            }
        }
        active_query.for_each(|(active, _)| {commands.entity(active).despawn()});
    }
    if keyboard_input.just_pressed(KeyCode::Return) || keyboard_input.just_pressed(KeyCode::Z) {
        for (button, _) in button_query.iter_mut() {
            if button.is_active {
                if button.id == 1 {
                    game_state.set(GameState::Started).unwrap();
                }
                else if button.id == 2 {
                    exit.send(AppExit);
                }
            }
        }
    }
}

fn menu_cleanup(button_query: Query<(Entity, &Button)>, active_query: Query<(Entity, &Active)>, mut commands: Commands) {
    button_query.for_each(|(button, _)| {commands.entity(button).despawn()});
    active_query.for_each(|(active, _)| {commands.entity(active).despawn()});
}