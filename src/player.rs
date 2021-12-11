use super::camera::new_camera_2d;
use super::components::{Jumper, Materials, Player};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("player_setup", SystemStage::single(spawn_player.system()))
            .add_system(player_jumps.system())
            .add_system(player_movement.system())
            .add_system(jump_reset.system());
    }
}

pub fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    let rigid_body = RigidBodyBundle {
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        activation: RigidBodyActivation::cannot_sleep(),
        forces: RigidBodyForces {
            gravity_scale: 3.,
            ..Default::default()
        },
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.5, 0.5),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        },
        ..Default::default()
    };
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_material.clone(),
            sprite: Sprite::new(Vec2::new(1.0, 1.0)),
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Player { speed: 7. })
        .insert(Jumper {
            jump_impulse: 14.,
            is_jumping: false,
        })
        .with_children(|parent| {
            parent.spawn_bundle(new_camera_2d());
        });
}

pub fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Jumper, &mut RigidBodyVelocity), With<Player>>,
) {
    for (mut jumper, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) && !jumper.is_jumping {
            velocity.linvel = Vec2::new(0., jumper.jump_impulse).into();
            jumper.is_jumping = true
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut RigidBodyVelocity)>,
) {
    for (player, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.linvel = Vec2::new(-player.speed, velocity.linvel.y).into();
        }
        if keyboard_input.pressed(KeyCode::Right) {
            velocity.linvel = Vec2::new(player.speed, velocity.linvel.y).into();
        }
    }
}

pub fn jump_reset(
    mut query: Query<(Entity, &mut Jumper)>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for contact_event in contact_events.iter() {
        for (entity, mut jumper) in query.iter_mut() {
            if let ContactEvent::Started(h1, h2) = contact_event {
                if h1.entity() == entity || h2.entity() == entity {
                    jumper.is_jumping = false
                }
            }
        }
    }
}
