mod components;
pub use components::*;
mod camera;
pub use camera::*;
mod player;
pub use player::*;
mod map;
pub use map::*;
mod monsters;
pub use monsters::*;
mod bullets;
pub use bullets::*;
mod living_beings;
pub use living_beings::*;
mod monster_ai;
pub use monster_ai::*;

use super::AppState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_floor.system()))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(back_to_main_menu_controls.system()),
            )
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(PlayerPlugin)
            .add_plugin(MonsterAiPlugin)
            .add_startup_system(setup.system());
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(Materials {
        player_material: materials.add(Color::rgb(0.969, 0.769, 0.784).into()),
        floor_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        monster_material: materials.add(Color::rgb(0.8, 0., 0.).into()),
        bullet_material: materials.add(Color::rgb(0.8, 0.8, 0.).into()),
    });
}

fn back_to_main_menu_controls(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if *app_state.current() == AppState::InGame {
        if keys.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::MainMenu).unwrap();
            keys.reset(KeyCode::Escape);
        }
    }
}
