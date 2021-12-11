use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
mod components;
pub use components::*;
mod camera;
pub use camera::*;
mod player;
pub use player::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Platformer!".to_string(),
            width: 640.0,
            height: 400.0,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_stage("player_setup", SystemStage::single(spawn_player.system()))
        .add_startup_stage("floor_setup", SystemStage::single(spawn_floor.system()))
        .add_system(player_jumps.system())
        .add_system(player_movement.system())
        .add_system(jump_reset.system())
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_floor(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let width = 10.;
    let height = 1.;
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(0.0, -2.).into(),
        body_type: RigidBodyType::Static,
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(width / 2., height / 2.),
        ..Default::default()
    };
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
            sprite: Sprite::new(Vec2::new(width, height)),
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete);
}

fn jump_reset(
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