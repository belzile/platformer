use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Materials, Bullet, GameDirection, Enemy};

pub struct BulletOptions {
  pub x: f32,
  pub y: f32,
  pub direction: GameDirection,
}

pub fn insert_bullet_at(commands: &mut Commands, materials: &Res<Materials>, options: BulletOptions) {
  let speed = match options.direction {
    GameDirection::Left => -14.0,
    _ => 14.0
  };

  let x = match options.direction {
    GameDirection::Left => options.x - 1.,
    _ => options.x + 1.
  };
  let rigid_body = RigidBodyBundle {
    position: Vec2::new(x, options.y).into(),
    velocity: RigidBodyVelocity {
      linvel: Vec2::new(speed, 0.0).into(),
      ..Default::default()
    },
    mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
    activation: RigidBodyActivation::cannot_sleep(),
    forces: RigidBodyForces {
        gravity_scale: 0.,
        ..Default::default()
    },
    ..Default::default()
};

let collider = ColliderBundle {
    shape: ColliderShape::cuboid(0.25, 0.05),
    flags: ColliderFlags {
        active_events: ActiveEvents::CONTACT_EVENTS,
        ..Default::default()
    },
    ..Default::default()
};

let sprite = SpriteBundle {
    material: materials.monster_material.clone(),
    sprite: Sprite::new(Vec2::new(0.5, 0.1)),
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
    mut commands: Commands,
    bullets: Query<Entity, With<Bullet>>,
    enemies: Query<Entity, With<Enemy>>,
    mut contact_events: EventReader<ContactEvent>,
  ) {
      for contact_event in contact_events.iter() {
          if let ContactEvent::Started(h1, h2) = contact_event {
              for bullet in bullets.iter() {
                  for enemy in enemies.iter() {
                      if (h1.entity() == bullet && h2.entity() == enemy) || (h1.entity() == enemy && h2.entity() == bullet) {
                          commands.entity(enemy).despawn_recursive();
                      }
                  }
              }
          }
      }
  }
