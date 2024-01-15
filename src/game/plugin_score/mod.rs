use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>()
            .init_resource::<Score>()
            .add_systems(
                Update,
                (
                    update_score.run_if(in_state(GameState::GameRunning)),
                    update_highscores,
                    highscores_updated,
                ),
            );
    }
}
