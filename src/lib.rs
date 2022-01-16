use bevy::{prelude::*, window::WindowMode};
use wasm_bindgen::prelude::*;

mod game;
use game::GamePlugin;

mod menus;
use menus::MenusPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    GameOver,
    BetweenLevels,
}

#[wasm_bindgen]
pub fn run() {
    let mut app = App::new();

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.insert_resource(WindowDescriptor {
        title: "Platformer!".to_string(),
        width: 640.0,
        height: 400.0,
        vsync: true,
        mode: WindowMode::Windowed,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_state(AppState::MainMenu)
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugin(MenusPlugin)
    .add_plugin(GamePlugin)
    .run();
}
