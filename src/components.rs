use bevy::{prelude::Handle, sprite::ColorMaterial};

pub struct Materials {
    pub player_material: Handle<ColorMaterial>,
    pub floor_material: Handle<ColorMaterial>,
}

pub struct Player {
    pub speed: f32,
}

pub struct Jumper {
    pub jump_impulse: f32,
    pub is_jumping: bool,
}
