use bevy::{prelude::Handle, sprite::ColorMaterial};

pub struct Materials {
    pub player_material: Handle<ColorMaterial>,
    pub floor_material: Handle<ColorMaterial>,
    pub monster_material: Handle<ColorMaterial>,
    pub bullet_material: Handle<ColorMaterial>,
}

#[derive(Copy, Clone)]
pub enum GameDirection {
    Left,
    Right,
}

pub struct Player {
    pub speed: f32,
    pub facing_direction: GameDirection,
}

pub struct Enemy;

pub struct Monster {
    pub speed: f32,
    pub facing_direction: GameDirection,
}

pub struct Bullet;

pub struct Jumper {
    pub jump_impulse: f32,
    pub is_jumping: bool,
}
