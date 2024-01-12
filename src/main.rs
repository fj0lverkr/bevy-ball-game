use bevy::prelude::*;

mod game;
mod main_menu;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins((DefaultPlugins, MainMenuPlugin, GamePlugin))
        .run();
}

#[derive(States, Default, Hash, Debug, Eq, PartialEq, Clone)]
enum GameState {
    #[default]
    MainMenu,
    GameRunning,
    GameOver,
}
