use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

use super::super::AppState;
use super::{GameDirection, Jumper, Monster};

struct MonsterWalkedIntoWallEvent {
    entity: Entity,
}

pub struct MonsterAiPlugin;

impl Plugin for MonsterAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MonsterWalkedIntoWallEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(monster_walking_system.system())
                    .with_system(monster_wall_contact_detection.system())
                    .with_system(monster_change_direction_on_contact.system()),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(2.0))
                    .with_system(monster_jumps.system()),
            );
    }
}

fn monster_walking_system(mut monsters: Query<(&Monster, &mut RigidBodyVelocityComponent)>) {
    for (monster, mut velocity) in monsters.iter_mut() {
        let speed = match monster.facing_direction {
            GameDirection::Left => -monster.speed,
            GameDirection::Right => monster.speed,
        };

        velocity.linvel = Vec2::new(speed, velocity.linvel.y).into();
    }
}

fn monster_wall_contact_detection(
    monsters: Query<Entity, With<Monster>>,
    mut contact_events: EventReader<ContactEvent>,
    mut send_monster_walked_into_wall: EventWriter<MonsterWalkedIntoWallEvent>,
) {
    for contact_event in contact_events.iter() {
        if let ContactEvent::Started(h1, h2) = contact_event {
            for monster in monsters.iter() {
                if h1.entity() == monster || h2.entity() == monster {
                    send_monster_walked_into_wall
                        .send(MonsterWalkedIntoWallEvent { entity: monster })
                }
            }
        }
    }
}

fn monster_change_direction_on_contact(
    mut events: EventReader<MonsterWalkedIntoWallEvent>,
    mut monster_query: Query<&mut Monster>,
) {
    for event in events.iter() {
        // bullet contacts may destroy monster before running this system.
        if let Ok(mut monster) = monster_query.get_mut(event.entity) {
            monster.facing_direction = match monster.facing_direction {
                GameDirection::Left => GameDirection::Right,
                GameDirection::Right => GameDirection::Left,
            }
        }
    }
}

fn monster_jumps(mut monsters: Query<(&mut Jumper, &mut RigidBodyVelocityComponent), With<Monster>>) {
    for (monster, mut velocity) in monsters.iter_mut() {
        if should_jump() {
            velocity.linvel = Vec2::new(0., monster.jump_impulse).into();
        }
    }
}

fn should_jump() -> bool {
    let mut rng = thread_rng();
    rng.gen_bool(0.1)
}
