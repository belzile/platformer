use super::{components::Materials, insert_monster_at, WinningZone};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

pub fn spawn_floor(mut commands: Commands, materials: Res<Materials>) {
    let world = create_world(150);
    add_sprites(&mut commands, &materials, &world);
    add_colliders(&world, &mut commands);

    add_enemies(&mut commands, &world, &materials);
    add_winning_zone(&mut commands, &materials, 150.)
}

fn add_sprites(commands: &mut Commands, materials: &Res<Materials>, world: &Vec<usize>) {
    world.iter().enumerate().for_each(|(x, height)| {
        add_tile(commands, materials, x as f32, *height);
    });
}

fn add_enemies(commands: &mut Commands, world: &Vec<usize>, materials: &Res<Materials>) {
    world.iter().enumerate().for_each(|(x, height)| {
        if should_add_enemy(x) {
            insert_monster_at(commands, x, *height + 1, materials)
        }
    })
}

fn should_add_enemy(x: usize) -> bool {
    if x <= 5 {
        return false;
    }
    let mut rng = thread_rng();
    let random_number: u32 = rng.gen_range(0..100);
    match random_number {
        0..=90 => false,
        _ => true,
    }
}

fn create_world(width: usize) -> Vec<usize> {
    let mut heights: Vec<usize> = Vec::with_capacity(width);
    let mut height = 1;
    (0..width).for_each(|_| {
        heights.push(height);
        height = get_next_height(height)
    });
    heights
}

fn get_next_height(current_height: usize) -> usize {
    let next_height = current_height as isize + get_random_height_delta();
    return if next_height > 0 {
        next_height as usize
    } else {
        1
    };
}

fn get_random_height_delta() -> isize {
    let mut rng = thread_rng();
    let random_number: u32 = rng.gen_range(0..100);
    let delta = match random_number {
        0..=75 => 0,
        76..=81 => -1,
        82..=95 => 1,
        _ => 2,
    };
    delta
}

fn add_tile(commands: &mut Commands, materials: &Res<Materials>, x: f32, height: usize) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: materials.floor_material.clone(),
            custom_size: Vec2::new(1., height as f32).into(),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(x, height as f32 / 2. + 0.5, 0.)),
        ..Default::default()
    });
}

fn add_colliders(world: &Vec<usize>, commands: &mut Commands) {
    let max = match world.iter().max() {
        Some(m) => m,
        _ => panic!("add_colliders: World is empty"),
    };
    (1..=*max).for_each(|floor_height| {
        let mut start: Option<usize> = None;
        world
            .iter()
            .enumerate()
            .for_each(|(index, height_at_index)| {
                if *height_at_index >= floor_height && start.is_none() {
                    start = Some(index);
                } else if *height_at_index < floor_height && start.is_some() {
                    add_collider(commands, floor_height, *start.get_or_insert(0), index);
                    start = None
                }
            });

        if start.is_some() {
            add_collider(commands, floor_height, *start.get_or_insert(0), world.len());
        }
    })
}

fn add_collider(commands: &mut Commands, height: usize, from: usize, to: usize) {
    let width = to - from;
    let half_width = width as f32 / 2.;
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(from as f32 + half_width - 0.5, height as f32).into(),
        body_type: RigidBodyType::Static.into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(half_width, 0.5).into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete);
}

fn add_winning_zone(commands: &mut Commands, materials: &Res<Materials>, x: f32) {
    let height = 800.;
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(x, 0.).into(),
        body_type: RigidBodyType::Static.into(),
        ..Default::default()
    };

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.5, height / 2.).into(),
        ..Default::default()
    };

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: materials.winning_zone_material.clone(),
                custom_size: Vec2::new(1., height).into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(WinningZone);
}
