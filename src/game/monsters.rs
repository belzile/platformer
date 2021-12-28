use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Enemy, Materials, Monster};

pub fn insert_monster_at(commands: &mut Commands, x: usize, y: usize, materials: &Res<Materials>) {
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(x as f32, y as f32).into(),
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        activation: RigidBodyActivation::cannot_sleep(),
        forces: RigidBodyForces {
            gravity_scale: 3.,
            ..Default::default()
        },
        ..Default::default()
    };

    let collider = ColliderBundle {
        shape: ColliderShape::round_cuboid(0.35, 0.35, 0.1),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        },
        ..Default::default()
    };

    let sprite = SpriteBundle {
        material: materials.monster_material.clone(),
        sprite: Sprite::new(Vec2::new(0.9, 0.9)),
        ..Default::default()
    };

    commands
        .spawn_bundle(sprite)
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Enemy)
        .insert(Monster);
}

pub struct MonsterHitEvent {
    pub entity: Entity,
}

pub struct MonsterDeathEvent {
    entity: Entity,
}

pub fn on_monster_hit(
    mut monster_hit_events: EventReader<MonsterHitEvent>,
    mut send_monster_death: EventWriter<MonsterDeathEvent>,
) {
    for event in monster_hit_events.iter() {
        send_monster_death.send(MonsterDeathEvent {
            entity: event.entity,
        })
    }
}

pub fn on_monster_dead(mut monster_death_events: EventReader<MonsterDeathEvent>, mut commands: Commands) {
    for event in monster_death_events.iter() {
        commands.entity(event.entity).despawn_recursive()
    }
}
