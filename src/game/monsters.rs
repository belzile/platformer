use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Enemy, GameDirection, Jumper, LivingBeing, Materials, Monster};

pub fn insert_monster_at(commands: &mut Commands, x: usize, y: usize, materials: &Res<Materials>) {
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(x as f32, y as f32).into(),
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        activation: RigidBodyActivation::cannot_sleep().into(),
        forces: RigidBodyForces {
            gravity_scale: 3.,
            ..Default::default()
        }.into(),
        ..Default::default()
    };

    let collider = ColliderBundle {
        shape: ColliderShape::round_cuboid(0.35, 0.35, 0.1).into(),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }.into(),
        ..Default::default()
    };

    let sprite = SpriteBundle {
        sprite: Sprite {
            color: materials.monster_material.clone(),
            custom_size: Vec2::new(0.9, 0.9).into(),
            ..Default::default()
        },
        ..Default::default()
    };

    commands
        .spawn_bundle(sprite)
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(LivingBeing)
        .insert(Enemy)
        .insert(Monster {
            speed: 3.,
            facing_direction: GameDirection::Right,
        })
        .insert(Jumper {
            jump_impulse: 14.,
            is_jumping: false,
        });
}
