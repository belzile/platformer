use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Bullet, GameDirection, LivingBeing, LivingBeingHitEvent, Materials};

pub struct BulletFiredEvent {
    pub position: Vec2,
    pub direction: GameDirection,
}

pub fn on_bullet_fired(
    mut commands: Commands,
    materials: Res<Materials>,
    mut bullet_fired_events: EventReader<BulletFiredEvent>,
) {
    for event in bullet_fired_events.iter() {
        insert_bullet_at(&mut commands, &materials, event)
    }
}

pub fn insert_bullet_at(
    commands: &mut Commands,
    materials: &Res<Materials>,
    options: &BulletFiredEvent,
) {
    let speed = match options.direction {
        GameDirection::Left => -14.0,
        _ => 14.0,
    };

    let x = match options.direction {
        GameDirection::Left => options.position.x - 1.,
        _ => options.position.x + 1.,
    };
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(x, options.position.y).into(),
        velocity: RigidBodyVelocity {
            linvel: Vec2::new(speed, 0.0).into(),
            ..Default::default()
        }.into(),
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        activation: RigidBodyActivation::cannot_sleep().into(),
        forces: RigidBodyForces {
            gravity_scale: 0.,
            ..Default::default()
        }.into(),
        ..Default::default()
    };

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.25, 0.05).into(),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }.into(),
        ..Default::default()
    };

    let sprite = SpriteBundle {
        sprite: Sprite {
            color: materials.bullet_material.clone(),
            custom_size: Vec2::new(0.5, 0.1).into(),
            ..Default::default()
        },
        ..Default::default()
    };

    commands
        .spawn_bundle(sprite)
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Bullet);
}

pub fn destroy_bullet_on_contact(
    mut commands: Commands,
    bullets: Query<Entity, With<Bullet>>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for contact_event in contact_events.iter() {
        if let ContactEvent::Started(h1, h2) = contact_event {
            for bullet in bullets.iter() {
                if h1.entity() == bullet || h2.entity() == bullet {
                    commands.entity(bullet).despawn_recursive();
                }
            }
        }
    }
}

pub fn kill_on_contact(
    mut send_living_being_hit: EventWriter<LivingBeingHitEvent>,
    bullets: Query<Entity, With<Bullet>>,
    living_being: Query<Entity, With<LivingBeing>>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for contact_event in contact_events.iter() {
        if let ContactEvent::Started(h1, h2) = contact_event {
            for bullet in bullets.iter() {
                for enemy in living_being.iter() {
                    if (h1.entity() == bullet && h2.entity() == enemy)
                        || (h1.entity() == enemy && h2.entity() == bullet)
                    {
                        send_living_being_hit.send(LivingBeingHitEvent { entity: enemy });
                    }
                }
            }
        }
    }
}
