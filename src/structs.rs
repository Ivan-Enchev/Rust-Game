use bevy::{prelude::Bundle, math::Vec3, core::Timer};
use bevy_retrograde::prelude::*;
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

#[derive(PartialEq)]
pub enum Specialty {
    Poison, 
    StrongPoison,
    LongPoison,
    SlowPoison,
    FastPoison,
    ShortPoison,
    Death,
    Half,
    Weaken,
    SuperWeaken, 
    LowWeaken, 
    Paralize,
    Slow,
    SPNone
}

#[derive(PartialEq)]
pub enum Status {
    SNone,
    Protected,
    PoisonHeal,
    Weakened,
    SuperWeakened,
    LowWeakened
}

pub struct CurrentStatus {
    pub value: Status
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

#[derive(Bundle)]
pub struct AttackBundle {
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub attack: BasicAttack,
    pub layers: CollisionLayers,
    pub lvl_entity: LevelEntity
}
pub struct PlayerInventory {
    pub weapons: [Element; 2],
    pub active_weapon: i8,
    pub p_health: i16, 
    pub max_health: i16,
    pub coins: i32,
    pub p_element: Element,
    pub can_attack: bool
}

pub struct EChoice;

pub struct AttackSpecialty {
    pub value: Specialty
}
pub struct LevelEntity;
pub struct KeyDelay;
pub struct ProtectionDelay;
pub struct Room;
#[derive(Default)]
pub struct Damage {
    pub value: f32
}
pub struct Special1;

pub struct Special2;
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

#[derive(Default)]
pub struct Delay {
    pub timer: Timer
}

pub struct PoisonDelay {
    pub ticks: usize,
    pub timer: Timer
}

pub struct SPoisonDelay {
    pub ticks: usize,
    pub timer: Timer
}
pub struct DeathDelay {
    pub ticks: usize,
    pub timer: Timer
}

impl Delay {
    pub fn change_timer(&mut self, delay: f32) {
        self.timer = Timer::from_seconds(delay, true);
        self.timer.reset();
    }
}

impl PoisonDelay {
    pub fn finished(&self) -> bool {
        self.ticks <= 0
    }
}

impl SPoisonDelay {
    pub fn finished(&self) -> bool {
        self.ticks <= 0
    }
}

impl DeathDelay {
    pub fn finished(&self) -> bool {
        self.ticks <= 0
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

impl Default for AttackBundle {
    fn default() -> Self {
		Self {
            rigid_body: RigidBody::Sensor,
            velocity: Velocity::from_linear(Vec3::default()),
            attack: BasicAttack,
            layers: CollisionLayers::new(Layer::Projectile, Layer::Enemy),
            lvl_entity: LevelEntity
		}
	}
}