use bevy::prelude::*;

mod game;
mod game_over_screen;
mod main_menu;
mod pauze_screen;
mod shared;

use game::GamePlugin;
use game_over_screen::GameOverScreenPlugin;
use main_menu::MainMenuPlugin;
use pauze_screen::PauzeScreenPlugin;

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins((
            DefaultPlugins,
            MainMenuPlugin,
            PauzeScreenPlugin,
            GameOverScreenPlugin,
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
