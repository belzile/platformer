use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
mod components;
pub use components::*;
mod camera;
pub use camera::*;
mod player;
pub use player::*;
mod map;
pub use map::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Platformer!".to_string(),
            width: 640.0,
            height: 400.0,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_state(AppState::MainMenu)
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(spawn_floor.system())
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame)
            .with_system(cleanup_map.system())
        )
        // .add_startup_stage("floor_setup", SystemStage::single(spawn_floor.system()))
        .add_plugin(PlayerPlugin)
        .add_system(setup.system())
        .add_system(main_menu_controls.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(Materials {
        player_material: materials.add(Color::rgb(0.969, 0.769, 0.784).into()),
        floor_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
}

fn main_menu_controls(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if *app_state.current() == AppState::MainMenu {
        if keys.just_pressed(KeyCode::Return) {
            app_state.set(AppState::InGame).unwrap();
            keys.reset(KeyCode::Return);
        }
    } else {
        if keys.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::MainMenu).unwrap();
            keys.reset(KeyCode::Escape);
        }
    }
}
