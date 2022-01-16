use bevy::prelude::*;
use bevy_rapier2d::prelude::{RigidBodyPositionComponent};

use crate::AppState;

use super::Player;

#[derive(Component)]
pub struct LivingBeing;

pub struct LivingBeingHitEvent {
    pub entity: Entity,
}

pub struct LivingBeingDeathEvent {
    pub entity: Entity,
}

pub fn on_living_being_hit(
    mut living_being_hit_events: EventReader<LivingBeingHitEvent>,
    mut send_living_being_death: EventWriter<LivingBeingDeathEvent>,
) {
    for event in living_being_hit_events.iter() {
        send_living_being_death.send(LivingBeingDeathEvent {
            entity: event.entity,
        })
    }
}

pub fn on_living_being_dead(
    mut living_being_death_events: EventReader<LivingBeingDeathEvent>,
    player_query: Query<Entity, With<Player>>,
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
) {
    for event in living_being_death_events.iter() {
        let player_id = player_query.get_single();
        commands.entity(event.entity).despawn_recursive();
        match player_id {
            Ok(player) if event.entity == player => app_state.set(AppState::GameOver).unwrap(),
            _ => (),
        };
    }
}

pub fn death_by_height(
    mut send_death_event: EventWriter<LivingBeingDeathEvent>,
    living_being: Query<(Entity, &RigidBodyPositionComponent), With<LivingBeing>>,
) {
    for (entity, position) in living_being.iter() {
        if position.position.translation.y < -1. {
            send_death_event.send(LivingBeingDeathEvent { entity })
        }
    }
}
