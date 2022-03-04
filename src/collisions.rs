use std::time::Instant;

use crate::structs::*;
use bevy::prelude::*;
use bevy_retrograde::prelude::*;
use crate::Status::*;

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
        if status.value == Protection {
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
mut enemy_query: Query<(Entity, &mut Health, &mut CurrentStatus), (With<Enemy>, Without<Player>)>,
damage: Query<(Entity, &Damage, &Specialty)>, mut commands: Commands) {
    for event in events.iter().filter(|e| e.is_started()) {   
        let (layers_1, layers_2) = event.collision_layers();
        let (entity_1, entity_2) = event.rigid_body_entities();
        let mut effect_multiplier = 1.;
        if is_projectile(layers_1) && is_enemy(layers_2) {
            if damage.get_component::<Specialty>(entity_1).unwrap().value.as_str() == "poison" {
                let (current_entity, _, _) = enemy_query.get_mut(entity_2).unwrap();
                commands.entity(current_entity).insert(PoisonDelay{start: Instant::now(), ticks: 0});
            }
            else if damage.get_component::<Specialty>(entity_1).unwrap().value.as_str() == "weaken" {
                enemy_query.get_component_mut::<CurrentStatus>(entity_2).unwrap().value = Weakened;  
            } 
            else {
                if enemy_query.get_component::<CurrentStatus>(entity_2).unwrap().value == Weakened {
                    effect_multiplier = 2.;
                }
                let dealt_damage = damage.get_component::<Damage>(entity_1).unwrap().value * effect_multiplier;
                enemy_query.get_component_mut::<Health>(entity_2).unwrap().value -= dealt_damage as i16;
            }
        } else if is_projectile(layers_2) && is_enemy(layers_1) {
            if damage.get_component::<Specialty>(entity_2).unwrap().value.as_str() == "poison" {
                let (current_entity, _, _) = enemy_query.get_mut(entity_1).unwrap();
                commands.entity(current_entity).insert(PoisonDelay{start: Instant::now(), ticks: 0});
            }
            else if damage.get_component::<Specialty>(entity_2).unwrap().value.as_str() == "weaken" {
                enemy_query.get_component_mut::<CurrentStatus>(entity_1).unwrap().value = Weakened;  
            } 
            else {
                if enemy_query.get_component::<CurrentStatus>(entity_1).unwrap().value == Weakened {
                    effect_multiplier = 2.;
                }
                let dealt_damage = damage.get_component::<Damage>(entity_2).unwrap().value * effect_multiplier;
                enemy_query.get_component_mut::<Health>(entity_1).unwrap().value -= dealt_damage as i16;
            }
        }
    }  
}