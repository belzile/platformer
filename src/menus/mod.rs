use super::AppState;
use bevy::{app::AppExit, prelude::*};

mod common;
pub use common::*;

pub struct MenusPlugin;

#[derive(Component)]
enum MenuButton {
    Play,
    BackToMainMenu,
    Quit,
}

fn button_press_system(
    buttons: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => state
                    .set(AppState::InGame)
                    .expect("Couldn't switch state to InGame"),
                MenuButton::BackToMainMenu => state
                    .set(AppState::MainMenu)
                    .expect("Couldn't switch state to MainMenu"),
                MenuButton::Quit => exit.send(AppExit),
            };
        }
    }
}

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuMaterials>()
            .add_system(button_system.system())
            .add_system(button_press_system.system())
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(cleanup.system())
                    .with_system(setup_main_menu.system()),
            )
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup.system()))
            .add_system_set(
                SystemSet::on_enter(AppState::GameOver)
                    .with_system(cleanup.system())
                    .with_system(setup_game_over_menu.system()),
            )
            .add_system_set(SystemSet::on_exit(AppState::GameOver).with_system(cleanup.system()))
            .add_system_set(
                SystemSet::on_enter(AppState::BetweenLevels)
                    .with_system(cleanup.system())
                    .with_system(setup_level_success_menu.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::BetweenLevels).with_system(cleanup.system()),
            );
    }
}

fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<MenuMaterials>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(root(&materials))
        .with_children(|parent| {
            parent
                .spawn_bundle(border(&materials))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(menu_background(&materials))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(
                                        &asset_server,
                                        &materials,
                                        "New Game",
                                    ));
                                })
                                .insert(MenuButton::Play);
                            if !cfg!(target_arch = "wasm32") {
                                parent
                                    .spawn_bundle(button(&materials))
                                    .with_children(|parent| {
                                        parent.spawn_bundle(button_text(
                                            &asset_server,
                                            &materials,
                                            "Quit",
                                        ));
                                    })
                                    .insert(MenuButton::Quit);
                            }
                        });
                });
        });
}

fn setup_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<MenuMaterials>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(root(&materials))
        .with_children(|parent| {
            parent.spawn_bundle(button_text(&asset_server, &materials, "Game Over"));
            parent
                .spawn_bundle(border(&materials))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(menu_background(&materials))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(
                                        &asset_server,
                                        &materials,
                                        "Replay",
                                    ));
                                })
                                .insert(MenuButton::Play);
                            parent
                                .spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(
                                        &asset_server,
                                        &materials,
                                        "Back to Main Menu",
                                    ));
                                })
                                .insert(MenuButton::BackToMainMenu);
                        });
                });
        });
}

fn setup_level_success_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<MenuMaterials>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(root(&materials))
        .with_children(|parent| {
            parent.spawn_bundle(button_text(&asset_server, &materials, "Level Success"));
            parent
                .spawn_bundle(border(&materials))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(menu_background(&materials))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(
                                        &asset_server,
                                        &materials,
                                        "Next Level",
                                    ));
                                })
                                .insert(MenuButton::Play);
                            parent
                                .spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(
                                        &asset_server,
                                        &materials,
                                        "Back to Main Menu",
                                    ));
                                })
                                .insert(MenuButton::BackToMainMenu);
                        });
                });
        });
}

fn cleanup(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
