use bevy_retrograde::prelude::*;
use std::time::Instant; 


#[derive(PhysicsLayer)]
pub enum Layer {
    Enemy,
    Player,
    Projectile,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    Started,
}

pub struct Active;
pub struct Button {
    pub is_active: bool,
    pub id: i8,
}
pub struct Enemy;
pub struct FlameSpirit;
pub struct Slime;
pub struct Player;
pub struct BasicAttack;
pub struct Speed {
    pub value: f32
}
pub struct Health {
    pub value: i8,
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

