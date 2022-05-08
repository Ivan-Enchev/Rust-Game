use crate::structs::*;
use bevy::prelude::*;
use bevy_retrograde::prelude::*;
use crate::Status::*;
use crate::Specialty::*;

pub fn despawn_defeated(mut commands: Commands, query: Query<(&Health, Entity)>, stage_query: Query<&mut GameStage>) {
    for (health, entity) in query.iter() {
        if health.value <= 0 {
            commands.entity(entity).despawn();
            stage_query.for_each_mut(|mut stage|{stage.enemies -= 1});
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

pub fn detect_collisions(mut events: EventReader<CollisionEvent>, mut player_health_query: Query<(&mut Health, &mut CurrentStatus), With<Player>>,
damage_query: Query<(Entity, &Damage), (With<Enemy>, Without<Player>)>) {

    for event in events.iter().filter(|e| e.is_started()) {    
        let (mut health, status) = if let Some((health, status)) = player_health_query.iter_mut().next() { (health, status) } else { return; };
        let (layers_1, layers_2) = event.collision_layers();
        let (entity_1, entity_2) = event.rigid_body_entities();
        let mut status_multiplier = 1.;
        if status.value == Protected {
            status_multiplier = 0.5;
        }

        if is_player(layers_1) && is_enemy(layers_2) {
            health.value -= (damage_query.get_component::<Damage>(entity_2).unwrap().value * status_multiplier) as i16;
            println!("\nHealth {}", health.value);
        } else if is_player(layers_2) && is_enemy(layers_1) {
            health.value -= (damage_query.get_component::<Damage>(entity_1).unwrap().value * status_multiplier) as i16;
            println!("\nHealth {}", health.value);
        }   
    }  
}

pub fn detect_enemy_collisions(mut events: EventReader<CollisionEvent>, 
mut enemy_query: Query<(Entity, &mut Health, &mut CurrentStatus, &mut Speed), (With<Enemy>, Without<Player>)>,
damage: Query<(Entity, &Damage, &AttackSpecialty)>, mut commands: Commands) {
    for event in events.iter().filter(|e| e.is_started()) {   
        let (layers_1, layers_2) = event.collision_layers();
        let (entity_1, entity_2) = event.rigid_body_entities();
        let mut effect_multiplier = 1.;
        if is_projectile(layers_1) && is_enemy(layers_2) {
            let (current_entity, _, _, _) = enemy_query.get_mut(entity_2).unwrap();
            match damage.get_component::<AttackSpecialty>(entity_1).unwrap().value {
                Poison => {commands.entity(current_entity).insert(PoisonDelay{timer: Timer::from_seconds(1., true), ticks: 5});},
                StrongPoison => {commands.entity(current_entity).insert(SPoisonDelay{timer: Timer::from_seconds(1., true), ticks: 5});},
                LongPoison => {commands.entity(current_entity).insert(PoisonDelay{timer: Timer::from_seconds(1., true), ticks: 10});},
                FastPoison => {commands.entity(current_entity).insert(PoisonDelay{timer: Timer::from_seconds(0.5, true), ticks: 5});},
                ShortPoison => {commands.entity(current_entity).insert(PoisonDelay{timer: Timer::from_seconds(1., true), ticks: 3});},
                SlowPoison => {commands.entity(current_entity).insert(PoisonDelay{timer: Timer::from_seconds(2., true), ticks: 100});},
                Death => {commands.entity(current_entity).insert(DeathDelay{timer: Timer::from_seconds(1., true), ticks: 10});},
                Weaken => {enemy_query.get_component_mut::<CurrentStatus>(entity_2).unwrap().value = Weakened;},
                LowWeaken => {enemy_query.get_component_mut::<CurrentStatus>(entity_2).unwrap().value = LowWeakened;},
                SuperWeaken => {enemy_query.get_component_mut::<CurrentStatus>(entity_2).unwrap().value = SuperWeakened;}
                Half => {enemy_query.get_component_mut::<Health>(entity_2).unwrap().value /= 2;},
                Paralize => {enemy_query.get_component_mut::<Speed>(entity_2).unwrap().value = 0.;},
                Slow => {enemy_query.get_component_mut::<Speed>(entity_2).unwrap().value /= 2.;},
                SPNone => {
                    if enemy_query.get_component::<CurrentStatus>(entity_2).unwrap().value == Weakened {
                        effect_multiplier *= 2.;
                    }
                    if enemy_query.get_component::<CurrentStatus>(entity_2).unwrap().value == SuperWeakened {
                        effect_multiplier *= 3.;
                    }
                    if enemy_query.get_component::<CurrentStatus>(entity_2).unwrap().value == LowWeakened {
                        effect_multiplier *= 1.5;
                    }
                    let dealt_damage = damage.get_component::<Damage>(entity_1).unwrap().value * effect_multiplier;
                    enemy_query.get_component_mut::<Health>(entity_2).unwrap().value -= dealt_damage as i16;
                }
            }
        } else if is_projectile(layers_2) && is_enemy(layers_1) {
            let (current_entity, _, _, _) = enemy_query.get_mut(entity_1).unwrap();
            match damage.get_component::<AttackSpecialty>(entity_2).unwrap().value {
                Poison => {commands.entity(current_entity).insert(PoisonDelay{timer: Timer::from_seconds(1., true), ticks: 5});},
                StrongPoison => {commands.entity(current_entity).insert(SPoisonDelay{timer: Timer::from_seconds(1., true), ticks: 5});},
                LongPoison => {commands.entity(current_entity).insert(PoisonDelay{timer: Timer::from_seconds(1., true), ticks: 10});},
                ShortPoison => {commands.entity(current_entity).insert(PoisonDelay{timer: Timer::from_seconds(1., true), ticks: 3});},
                SlowPoison => {commands.entity(current_entity).insert(PoisonDelay{timer: Timer::from_seconds(2., true), ticks: 100});},
                FastPoison => {commands.entity(current_entity).insert(PoisonDelay{timer: Timer::from_seconds(0.5, true), ticks: 5});},
                Death => {commands.entity(current_entity).insert(DeathDelay{timer: Timer::from_seconds(1., true), ticks: 10});},
                Weaken => {enemy_query.get_component_mut::<CurrentStatus>(entity_1).unwrap().value = Weakened;},
                LowWeaken => {enemy_query.get_component_mut::<CurrentStatus>(entity_1).unwrap().value = LowWeakened;},
                SuperWeaken => {enemy_query.get_component_mut::<CurrentStatus>(entity_1).unwrap().value = SuperWeakened;}
                Half => {enemy_query.get_component_mut::<Health>(entity_1).unwrap().value /= 2;},
                Paralize => {enemy_query.get_component_mut::<Speed>(entity_1).unwrap().value = 0.;},
                Slow => {enemy_query.get_component_mut::<Speed>(entity_1).unwrap().value /= 2.;},
                SPNone => {
                    if enemy_query.get_component::<CurrentStatus>(entity_1).unwrap().value == Weakened {
                        effect_multiplier *= 2.;
                    }
                    if enemy_query.get_component::<CurrentStatus>(entity_1).unwrap().value == SuperWeakened {
                        effect_multiplier *= 3.;
                    }
                    if enemy_query.get_component::<CurrentStatus>(entity_1).unwrap().value == LowWeakened {
                        effect_multiplier *= 1.5;
                    }
                    let dealt_damage = damage.get_component::<Damage>(entity_2).unwrap().value * effect_multiplier;
                    enemy_query.get_component_mut::<Health>(entity_1).unwrap().value -= dealt_damage as i16;
                }
            }
        }
    }  
}