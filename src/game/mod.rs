use bevy::prelude::*;

mod plugin_enemy;
mod plugin_gameloop;
mod plugin_player;
mod plugin_score;
mod plugin_star;
mod systems;

use plugin_enemy::EnemyPlugin;
use plugin_gameloop::GameLoopPlugin;
use plugin_player::PlayerPlugin;
use plugin_score::ScorePlugin;
use plugin_star::StarPlugin;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_plugins((
                GameLoopPlugin,
                EnemyPlugin,
                PlayerPlugin,
                ScorePlugin,
                StarPlugin,
            ))
            .add_systems(Update, toggle_simulation);
    }
}

#[derive(States, Default, Hash, Debug, Eq, PartialEq, Clone)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
