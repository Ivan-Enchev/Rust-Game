use crate::structs::*;
use bevy::prelude::*;
use bevy_retrograde::prelude::*;


pub fn despawn_defeated(mut commands: Commands, query: Query<(&Health, Entity)>) {
    for (health, entity) in query.iter() {
        if health.value <= 0 {
            commands.entity(entity).despawn();
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

pub fn detect_collisions(mut events: EventReader<CollisionEvent>, mut player_health_query: Query<&mut Health, With<Player>>) {

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

pub fn detect_enemy_collisions(mut events: EventReader<CollisionEvent>, 
    mut enemy_health_query: Query<(Entity, &mut Health), (With<Enemy>, Without<Player>)>) {
    for event in events.iter().filter(|e| e.is_started()) {   
        //let (entity, mut health) = if let Some((entity, health)) = enemy_health_query.iter_mut().next() {(entity, health)} else { return; };
        for (entity, mut health) in enemy_health_query.iter_mut() {
            let (layers_1, layers_2) = event.collision_layers();
            let (entity_1, entity_2) = event.rigid_body_entities();
            if is_projectile(layers_1) && is_enemy(layers_2) {
                if entity_2.id() == entity.id() {
                    health.value -= 1;
                    println!("Enemy Health {}", health.value);
                }
            } else if is_projectile(layers_2) && is_enemy(layers_1) {
                if entity_1.id() == entity.id() {
                    health.value -= 1;
                    println!("Enemy Health {}", health.value);
                }
            }
        }
    }  
}