use bevy::prelude::*;

mod plugin_enemy;
mod plugin_gameloop;
mod plugin_player;
mod plugin_score;
mod plugin_star;

use plugin_enemy::EnemyPlugin;
use plugin_gameloop::GameLoopPlugin;
use plugin_player::PlayerPlugin;
use plugin_score::ScorePlugin;
use plugin_star::StarPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameLoopPlugin,
            EnemyPlugin,
            PlayerPlugin,
            ScorePlugin,
            StarPlugin,
        ));
    }
}
