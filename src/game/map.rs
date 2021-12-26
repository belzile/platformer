use super::{components::Materials, insert_monster_at, Enemy};
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

pub struct MapData {
    map_entity: Entity,
}

pub fn spawn_floor(mut commands: Commands, materials: Res<Materials>) {
    let mut map_parent = commands.spawn();
    let world = create_world(150);
    add_sprites(&mut map_parent, &materials, &world);
    add_colliders(&world, &mut map_parent);
    let map_entity = map_parent.id();
    commands.insert_resource(MapData { map_entity });

    add_enemies(&mut commands, &world, &materials);
}

pub fn cleanup_map(
    mut commands: Commands,
    map_data: Res<MapData>,
    monsters: Query<Entity, With<Enemy>>,
) {
    commands.entity(map_data.map_entity).despawn_recursive();
    for monster in monsters.iter() {
        commands.entity(monster).despawn_recursive();
    }
}

fn add_sprites(commands: &mut EntityCommands, materials: &Res<Materials>, world: &Vec<usize>) {
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
        0..=70 => 0,
        71..=80 => -1,
        81..=90 => 1,
        _ => 2,
    };
    delta
}

fn add_tile(commands: &mut EntityCommands, materials: &Res<Materials>, x: f32, height: usize) {
    commands.with_children(|parent| {
        parent.spawn_bundle(SpriteBundle {
            material: materials.floor_material.clone(),
            sprite: Sprite::new(Vec2::new(1., height as f32)),
            global_transform: GlobalTransform::from_translation(Vec3::new(
                x,
                height as f32 / 2. + 0.5,
                0.,
            )),
            ..Default::default()
        });
    });
}

fn add_colliders(world: &Vec<usize>, commands: &mut EntityCommands) {
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

fn add_collider(commands: &mut EntityCommands, height: usize, from: usize, to: usize) {
    let width = to - from;
    let half_width = width as f32 / 2.;
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(from as f32 + half_width - 0.5, height as f32).into(),
        body_type: RigidBodyType::Static,
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(half_width, 0.5),
        ..Default::default()
    };
    commands.with_children(|parent| {
        parent
            .spawn_bundle(rigid_body)
            .insert_bundle(collider)
            .insert(RigidBodyPositionSync::Discrete);
    });
}
