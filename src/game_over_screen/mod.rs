mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::GameState;
use systems::layout::spawn_game_over_screen;

use self::systems::{
    interactions::{interact_with_quit_button, interact_with_restart_button},
    layout::despawn_game_over_screen,
};

pub struct GameOverScreenPlugin;

impl Plugin for GameOverScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_game_over_screen)
            .add_systems(
                Update,
                (interact_with_quit_button, interact_with_restart_button),
            )
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_screen);
    }
}
