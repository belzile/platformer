use super::super::AppState;
use super::Enemy;
use super::camera::new_camera_2d;
use super::components::{Jumper, Materials, Player};
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

struct PlayerData {
    player_entity: Entity,
    camera_entity: Entity,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(spawn_player.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(camera_follow_player.system())
                .with_system(player_jumps.system())
                .with_system(player_movement.system())
                .with_system(jump_reset.system())
                .with_system(death_by_height.system())
                .with_system(death_by_enemy.system()),
        )
        .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup_player.system()));
    }
}

fn cleanup_player(mut commands: Commands, player_data: Res<PlayerData>) {
    commands
        .entity(player_data.player_entity)
        .despawn_recursive();

    commands
        .entity(player_data.camera_entity)
        .despawn_recursive();
}

pub fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(0., 2.).into(),
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
    let player_entity = commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_material.clone(),
            sprite: Sprite::new(Vec2::new(0.9, 0.9)),
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
        .id();
    let camera_entity = commands.spawn_bundle(new_camera_2d()).id();
    commands.insert_resource(PlayerData {
        player_entity,
        camera_entity,
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
            set_jumping_false_if_touching_floor(entity, &mut jumper, contact_event);
        }
    }
}

fn set_jumping_false_if_touching_floor(entity: Entity, jumper: &mut Jumper, event: &ContactEvent) {
    if let ContactEvent::Started(h1, h2) = event {
        if h1.entity() == entity || h2.entity() == entity {
            jumper.is_jumping = false
        }
    }
}

fn camera_follow_player(
    mut cameras: Query<&mut Transform, With<Camera>>,
    players: Query<&RigidBodyPosition, With<Player>>,
) {
    for player in players.iter() {
        for mut camera in cameras.iter_mut() {
            camera.translation.x = player.position.translation.x;
            camera.translation.y = player.position.translation.y;
        }
    }
}

fn death_by_height(
    mut commands: Commands,
    players: Query<(Entity, &RigidBodyPosition), With<Player>>,
) {
    for (entity, position) in players.iter() {
        if position.position.translation.y < -1. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn death_by_enemy(
    mut commands: Commands,
    mut players: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for contact_event in contact_events.iter() {
        if let ContactEvent::Started(h1, h2) = contact_event {
            for player in players.iter_mut() {
                for enemy in enemies.iter() {
                    if (h1.entity() == player && h2.entity() == enemy) || (h1.entity() == enemy && h2.entity() == player) {
                        commands.entity(player).despawn_recursive();
                    }
                }
            }
        }
    }
}
