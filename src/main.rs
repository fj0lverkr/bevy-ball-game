use bevy::prelude::*;

mod game;
mod main_menu;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MainMenuPlugin, GamePlugin))
        .add_state::<GameState>()
        .run();
}

#[derive(States, Default, Hash, Debug, Eq, PartialEq, Clone)]
enum GameState {
    #[default]
    MainMenu,
    GameRunning,
    GameOver,
}
