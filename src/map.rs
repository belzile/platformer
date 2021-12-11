use super::components::Materials;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_floor(mut commands: Commands, materials: Res<Materials>) {
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
            material: materials.floor_material.clone(),
            sprite: Sprite::new(Vec2::new(width, height)),
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete);
}
