use bevy::app::AppExit;
use crate::structs::*;
use bevy::prelude::*;
use bevy_retrograde::prelude::*;
use crate::Element::*;
use rand::{thread_rng, Rng};


pub fn menu(keyboard_input: ResMut<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>, asset_server: Res<AssetServer>, 
    mut button_query: Query<(&mut Button, &GlobalTransform), With<Button>>, key_delay: Query<&mut Delay, With<KeyDelay>>,
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
    if keyboard_input.just_pressed(KeyCode::Z) {
        for (button, _) in button_query.iter_mut() {
            key_delay.for_each_mut(|mut delay|{delay.timer.reset()});
            if button.is_active {
                if button.id == 1 {
                    game_state.set(GameState::ElementSelect).unwrap();
                }
                else if button.id == 2 {
                    exit.send(AppExit);
                }
            }
        }
    }
}
    
pub fn menu_cleanup(button_query: Query<(Entity, &Button)>, active_query: Query<(Entity, &Active)>, mut commands: Commands) {
    button_query.for_each(|(button, _)| {commands.entity(button).despawn()});
    active_query.for_each(|(active, _)| {commands.entity(active).despawn()});
}

pub fn level_select(stage_query: Query<&mut GameStage>, arrow_query: Query<&mut GlobalTransform, With<ChoiceArrow>>,
keyboard_input: ResMut<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>, mut key_delay: Query<&mut Delay, With<KeyDelay>>, time:Res<Time>) {

    stage_query.for_each_mut(|mut stage| {
        
        if stage.active_room < 4 {
            if keyboard_input.just_pressed(KeyCode::Right) {
                if stage.active_room < stage.start_point + 1 {
                    stage.active_room += 1;
                    arrow_query.for_each_mut(|mut arrow| {arrow.translation.x += 30.});
                    stage.arrow_pos += 30.;
                }
            }
        }

        if stage.active_room > 0 {
            if keyboard_input.just_pressed(KeyCode::Left) {
                if stage.active_room > stage.start_point - 1 {
                    stage.active_room -= 1;
                    arrow_query.for_each_mut(|mut arrow| {arrow.translation.x -= 30.});
                    stage.arrow_pos -= 30.;
                }
            }
        }

        for mut delay in key_delay.iter_mut() {
            if delay.timer.tick(time.delta()).finished() {
                if keyboard_input.just_pressed(KeyCode::Z) {
                    stage.start_point = stage.active_room;
                    match stage.rooms_1[stage.active_room as usize] {
                        1 => game_state.set(GameState::Started).unwrap(),
                        2 => game_state.set(GameState::Shop).unwrap(),
                        3 => game_state.set(GameState::Heal).unwrap(),
                        4 => game_state.set(GameState::Started).unwrap(),
                        5 => game_state.set(GameState::ArtifactRoom).unwrap(),
                        _ => print!("No such room!"),
                    }
                    delay.timer.reset();
                }
            }
        }
    
    }) 
}

pub fn level_select_cleanup(arrow_query: Query<(Entity, &ChoiceArrow)>, rooms_query: Query<(Entity, &Room)>, mut commands: Commands) {    
    arrow_query.for_each(|(arrow, _)| {commands.entity(arrow).despawn()});
    rooms_query.for_each(|(room, _)| {commands.entity(room).despawn()});
}

pub fn level_end_system(mut game_state: ResMut<State<GameState>>, stage_query: Query<&mut GameStage>,
player_health_query: Query<&Health, With<Player>>, inventory: Query<&mut PlayerInventory>) {

    for player_health in player_health_query.iter() {
        inventory.for_each_mut(|mut inventory|{inventory.p_health = player_health.value});
    }

    stage_query.for_each_mut(|mut stage| {
        if stage.enemies <= 0 {
            let mut gain_coins = thread_rng();
            inventory.for_each_mut(|mut inventory|{inventory.coins = gain_coins.gen_range(1..=(stage.level as i32 * 5))});
            if stage.rooms_1[stage.active_room as usize] == 4 {
                inventory.for_each_mut(|mut inventory|{
                    inventory.coins = gain_coins.gen_range(1..=(stage.level as i32 * 10));
                    let weapon_gen = gain_coins.gen_range(1..=5);
                    match inventory.weapon_1 {
                        ENone => {
                            match weapon_gen {
                                1 => inventory.weapon_1 = Darkness,
                                2 => inventory.weapon_1 = Nature,
                                3 => inventory.weapon_1 = Air,
                                4 => inventory.weapon_1 = Water,
                                5 => inventory.weapon_1 = Fire,
                                _ => inventory.weapon_1 = ENone
                            }
                        },
                        _ => {
                            match weapon_gen {
                                1 => inventory.weapon_2 = Darkness,
                                2 => inventory.weapon_2 = Nature,
                                3 => inventory.weapon_2 = Air,
                                4 => inventory.weapon_2 = Water,
                                5 => inventory.weapon_2 = Fire,
                                _ => inventory.weapon_2 = ENone
                            }
                        }
                        //Need to add choice for weapon switch
                    }
                });
            }
            stage.next_level();
            game_state.set(GameState::LevelSelection).unwrap();
        }
    });
}

pub fn level_cleanup(query: Query<(Entity, &LevelEntity)>, mut commands: Commands, cam_query: Query<&mut Transform, With<Camera>>) {
    cam_query.for_each_mut(|mut camera| {camera.translation = Transform::from_xyz(0., 0., 0.).translation});
    query.for_each(|(entity, _)| {commands.entity(entity).despawn()});
}

pub fn heal(mut game_state: ResMut<State<GameState>>, stage_query: Query<&mut GameStage>, inventory: Query<&mut PlayerInventory>,
key_delay: Query<&mut Delay, With<KeyDelay>>) {
    inventory.for_each_mut(|mut inventory| {
        let mut gain_heatlh = thread_rng();
        inventory.p_health += gain_heatlh.gen_range(1..=4);
        if inventory.p_health > 10 {
            inventory.p_health = 10;
        }
        stage_query.for_each_mut(|mut stage| {stage.next_level();});
        key_delay.for_each_mut(|mut delay| {delay.timer.reset()});
        game_state.set(GameState::LevelSelection).unwrap();
    })
}

pub fn element_select(stage_query: Query<&mut GameStage>, arrow_query: Query<&mut GlobalTransform, With<ChoiceArrow>>,
keyboard_input: ResMut<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>, inventory: Query<&mut PlayerInventory>,
delay_query: QuerySet<(Query<&mut Delay, With<KeyDelay>>, Query<&mut Delay, With<Special1>>)>, time: Res<Time>) {

    let key_delay = delay_query.q0();
    let special_delay = delay_query.q1();
    
    stage_query.for_each_mut(|mut stage| {
        arrow_query.for_each_mut(|mut arrow| {
            match stage.active_room {
                0 => arrow.translation = Vec3::new(-60., -55., 0.),
                1 => arrow.translation = Vec3::new(0., -55., 0.),
                2 => arrow.translation = Vec3::new(60., -55., 0.),
                3 => arrow.translation = Vec3::new(-30., 5., 0.),
                4 => arrow.translation = Vec3::new(30., 5., 0.),
                _ => arrow.translation = Vec3::new(0., 0., 0.)
            }
        });

        if keyboard_input.just_pressed(KeyCode::Right) {
            stage.active_room += 1;
            if stage.active_room >= 5 {
                stage.active_room = 0;
            }
        }

        if keyboard_input.just_pressed(KeyCode::Left) {
            stage.active_room -= 1;
            if stage.active_room <= -1 {
                stage.active_room = 4;
            }
        }

        key_delay.for_each_mut(|mut delay| {
            if delay.timer.tick(time.delta()).finished() {
                if keyboard_input.just_pressed(KeyCode::Z) {
                    inventory.for_each_mut(|mut inventory| {
                        match stage.active_room {
                            0 => {
                                inventory.p_element = Darkness;
                                special_delay.for_each_mut(|mut delay| {delay.change_timer(15.)});
                            },
                            1 => {
                                inventory.p_element = Nature;
                                special_delay.for_each_mut(|mut delay| {delay.change_timer(10.)});
                            },
                            2 => {
                                inventory.p_element = Air;
                                special_delay.for_each_mut(|mut delay| {delay.change_timer(15.)});
                            },
                            3 => {
                                inventory.p_element = Water;
                                special_delay.for_each_mut(|mut delay| {delay.change_timer(30.)});
                            },
                            4 => {
                                inventory.p_element = Fire;
                                special_delay.for_each_mut(|mut delay| {delay.change_timer(10.)});
                            },
                            _ => inventory.p_element = ENone,
                        }
                    });
                    stage.active_room = 2;
                    delay.timer.reset();
                    game_state.set(GameState::LevelSelection).unwrap();
                }
            }
        })
    });
}

pub fn element_cleanup(element_query: Query<(Entity, &EChoice)>, mut commands: Commands, arrow_query: Query<(Entity, &ChoiceArrow)>) {
    arrow_query.for_each(|(arrow, _)| {commands.entity(arrow).despawn()});
    element_query.for_each(|(element, _)| {commands.entity(element).despawn()});
}