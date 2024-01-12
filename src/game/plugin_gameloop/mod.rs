use bevy::prelude::*;

pub mod events;
mod systems;

use events::*;
use systems::*;

pub struct GameLoopPlugin;

impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    handle_game_over,
                    transition_to_running_state,
                    handle_esc_pressed,
                ),
            );
    }
}
