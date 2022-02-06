use bevy::prelude::{Component, Color};

pub struct Materials {
    pub player_material: Color,
    pub floor_material: Color,
    pub monster_material: Color,
    pub bullet_material: Color,
    pub winning_zone_material: Color,
}

#[derive(Copy, Clone)]
pub enum GameDirection {
    Left,
    Right,
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub facing_direction: GameDirection,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Monster {
    pub speed: f32,
    pub facing_direction: GameDirection,
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Jumper {
    pub jump_impulse: f32,
    pub is_jumping: bool,
}

#[derive(Component)]
pub struct WinningZone;
