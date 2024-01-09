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

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            GameLoopPlugin,
            DefaultPlugins,
            PlayerPlugin,
            EnemyPlugin,
            StarPlugin,
            ScorePlugin,
        ))
        .run();
}
