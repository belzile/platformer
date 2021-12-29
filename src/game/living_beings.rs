use bevy::prelude::*;
use bevy_rapier2d::prelude::RigidBodyPosition;

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
    mut commands: Commands,
) {
    for event in living_being_death_events.iter() {
        commands.entity(event.entity).despawn_recursive()
    }
}

pub fn death_by_height(
    mut send_death_event: EventWriter<LivingBeingDeathEvent>,
    living_being: Query<(Entity, &RigidBodyPosition), With<LivingBeing>>,
) {
    for (entity, position) in living_being.iter() {
        if position.position.translation.y < -1. {
            send_death_event.send(LivingBeingDeathEvent { entity })
        }
    }
}
