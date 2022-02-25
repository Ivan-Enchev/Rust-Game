use bevy_retrograde::prelude::*;
use std::time::Instant; 
use rand::{thread_rng, Rng};


#[derive(PhysicsLayer)]
pub enum Layer {
    Enemy,
    Player,
    Projectile,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    LevelSelection,
    Started,
    Shop,
    Heal,
    ArtifactRoom,
    ElementSelect
}

pub enum Element {
    Darkness, 
    Nature,
    Fire,
    Water,
    Air,
    ENone
}
pub struct GameStage {
    pub level: i16,
    pub rooms_1: [i8; 5],
    pub rooms_2: [i8; 5],
    pub rooms_3: [i8; 5],
    pub active_room: i8,
    pub start_point: i8,
    pub arrow_pos: f32,
    pub enemies: i32
}

pub struct Active;
pub struct Button {
    pub is_active: bool,
    pub id: i8,
}

pub struct PlayerInventory {
    pub p_health: i16,
    pub p_element: Element
}

pub struct EChoice;
pub struct LevelEntity;
pub struct KeyDelay;
pub struct Room;

pub struct ChoiceArrow;
pub struct Enemy;
pub struct FlameSpirit;
pub struct Slime;
pub struct Player;
pub struct BasicAttack;
pub struct Speed {
    pub value: f32
}
pub struct Health {
    pub value: i16,
}

pub struct Delay {
    pub start: Instant,
    pub delay: f64,
}

impl Delay {
    pub fn next_action_aviable(&self, now: Instant) -> bool {
        now.duration_since(self.start).as_secs_f64() >= self.delay
    }
}

impl GameStage {
    pub fn next_level(&mut self) {
        let mut rng = thread_rng();
        let mut number: i16 = rng.gen_range(0..100);
        self.level += 1;
        self.rooms_1 = self.rooms_2;
        self.rooms_2 = self.rooms_3;
        for i in 0..5 {
            let mut room;
            match number {
                0..=59 => room = 1,
                60..=69 => room = 2,
                70..=79 => room = 3,
                80..=89 => room = 4,
                90..=99 => room = 5,
                _ => room = 0
            }
            if self.level < 10 && room == 5 {
                room = 1;
            }
            self.rooms_3[i] = room;
            number = rng.gen_range(0..90);
        }
    }
}

