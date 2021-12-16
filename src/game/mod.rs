mod components;
pub use components::*;
mod camera;
pub use camera::*;
mod player;
pub use player::*;
mod map;
pub use map::*;

use super::AppState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_floor.system()))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup_map.system()))
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(PlayerPlugin)
            .add_startup_system(setup.system());
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(Materials {
        player_material: materials.add(Color::rgb(0.969, 0.769, 0.784).into()),
        floor_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
}
