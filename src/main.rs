use bevy::prelude::*;

mod game;
mod main_menu;
mod pauze_screen;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use pauze_screen::PauzeScreenPlugin;

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins((
            DefaultPlugins,
            MainMenuPlugin,
            PauzeScreenPlugin,
            GamePlugin,
        ))
        .run();
}

#[derive(States, Default, Hash, Debug, Eq, PartialEq, Clone)]
enum GameState {
    #[default]
    MainMenu,
    GameRunning,
    GameOver,
}
