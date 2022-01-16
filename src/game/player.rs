use super::super::AppState;
use super::camera::new_camera_2d;
use super::components::{Jumper, Materials, Player};
use super::{
    death_by_height, destroy_bullet_on_contact, kill_on_contact, on_bullet_fired,
    on_living_being_dead, on_living_being_hit, BulletFiredEvent, Enemy, GameDirection, LivingBeing,
    LivingBeingDeathEvent, LivingBeingHitEvent,
};
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LivingBeingHitEvent>()
            .add_event::<LivingBeingDeathEvent>()
            .add_event::<BulletFiredEvent>()
            .add_system_set(
                SystemSet::on_enter(AppState::InGame).with_system(spawn_player.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(camera_follow_player.system())
                    .with_system(player_jumps.system())
                    .with_system(player_controller.system())
                    .with_system(jump_reset.system())
                    .with_system(death_by_height.system())
                    .with_system(death_by_enemy.system())
                    .with_system(fire_controller.system())
                    .with_system(kill_on_contact.system())
                    .with_system(destroy_bullet_on_contact.system())
                    .with_system(on_living_being_hit.system())
                    .with_system(on_living_being_dead.system())
                    .with_system(on_bullet_fired.system()),
            );
    }
}

pub fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(0., 2.).into(),
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
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: materials.player_material.clone(),
                custom_size: Vec2::new(0.9, 0.9).into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(LivingBeing)
        .insert(Player {
            speed: 7.,
            facing_direction: GameDirection::Right,
        })
        .insert(Jumper {
            jump_impulse: 14.,
            is_jumping: false,
        });
    commands.spawn_bundle(new_camera_2d());
}

pub fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Jumper, &mut RigidBodyVelocityComponent), With<Player>>,
) {
    for (mut jumper, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) && !jumper.is_jumping {
            velocity.linvel = Vec2::new(0., jumper.jump_impulse).into();
            jumper.is_jumping = true
        }
    }
}

pub fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Player, &mut RigidBodyVelocityComponent)>,
) {
    for (mut player, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.linvel = Vec2::new(-player.speed, velocity.linvel.y).into();
            player.facing_direction = GameDirection::Left
        }
        if keyboard_input.pressed(KeyCode::Right) {
            velocity.linvel = Vec2::new(player.speed, velocity.linvel.y).into();
            player.facing_direction = GameDirection::Right
        }
    }
}

pub fn fire_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut send_fire_event: EventWriter<BulletFiredEvent>,
    players: Query<(&Player, &RigidBodyPositionComponent), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (player, position) in players.iter() {
            let event = BulletFiredEvent {
                position: Vec2::new(
                    position.position.translation.x,
                    position.position.translation.y,
                ),
                direction: player.facing_direction,
            };
            send_fire_event.send(event);
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
    players: Query<&RigidBodyPositionComponent, With<Player>>,
) {
    for player in players.iter() {
        for mut camera in cameras.iter_mut() {
            camera.translation.x = player.position.translation.x;
            camera.translation.y = player.position.translation.y;
        }
    }
}

pub fn death_by_enemy(
    mut send_player_hit: EventWriter<LivingBeingHitEvent>,
    players: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for contact_event in contact_events.iter() {
        if let ContactEvent::Started(h1, h2) = contact_event {
            for player in players.iter() {
                for enemy in enemies.iter() {
                    if (h1.entity() == player && h2.entity() == enemy)
                        || (h1.entity() == enemy && h2.entity() == player)
                    {
                        send_player_hit.send(LivingBeingHitEvent { entity: player })
                    }
                }
            }
        }
    }
}
