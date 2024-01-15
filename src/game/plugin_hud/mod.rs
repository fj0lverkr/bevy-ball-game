pub mod components;
pub mod styles;
pub mod systems;

use bevy::prelude::*;

use crate::GameState;

use self::systems::{
    layout::{despawn_hud, spawn_hud},
    update_number_of_enemies, update_score_counter,
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameRunning), spawn_hud)
            .add_systems(Update, (update_number_of_enemies, update_score_counter))
            .add_systems(OnExit(GameState::GameRunning), despawn_hud);
    }
}
