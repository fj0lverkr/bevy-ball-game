mod components;
mod styles;
mod systems;

use bevy::prelude::*;
use systems::layout::{despawn_main_menu, spawn_main_menu};

use crate::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
    }
}
