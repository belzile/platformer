use super::components::Materials;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

pub fn spawn_floor(mut commands: Commands, materials: Res<Materials>) {
    let mut height = 1;
    for x in 0..150 {
        add_tile(&mut commands, &materials, x as f32, height);
        height = get_next_height(height)
    }
}

fn get_next_height(current_height: u8) -> u8 {
    let next_height = current_height as i8 + get_random_height_delta();
    return if next_height > 0 {
        next_height as u8
    } else {
        1
    };
}

fn get_random_height_delta() -> i8 {
    let mut rng = thread_rng();
    let random_number: u32 = rng.gen_range(0..100);
    let delta = match random_number {
        0..=70 => 0,
        71..=80 => -1,
        81..=90 => 1,
        _ => 2,
    };
    delta
}

fn add_tile(commands: &mut Commands, materials: &Res<Materials>, x: f32, height: u8) {
    let half_height = height as f32 / 2.;
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(x, -2. + half_height).into(),
        body_type: RigidBodyType::Static,
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.5, half_height),
        ..Default::default()
    };
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.floor_material.clone(),
            sprite: Sprite::new(Vec2::new(1., height as f32)),
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete);
}
