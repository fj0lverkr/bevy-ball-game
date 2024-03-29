use bevy::prelude::*;

use crate::game::SimulationState;

use systems::layout::spawn_pauze_screen;

use self::systems::{interactions::interact_with_continue_button, layout::despawn_pauze_screen};

mod components;
mod styles;
mod systems;

pub struct PauzeScreenPlugin;

impl Plugin for PauzeScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Paused), spawn_pauze_screen)
            .add_systems(Update, interact_with_continue_button)
            .add_systems(OnExit(SimulationState::Paused), despawn_pauze_screen);
    }
}
