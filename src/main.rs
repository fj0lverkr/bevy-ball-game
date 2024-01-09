mod events;
mod plugin_enemy;
mod plugin_player;
mod plugin_score;
mod plugin_star;
mod systems;

use events::*;
use systems::*;

use plugin_enemy::EnemyPlugin;
use plugin_player::PlayerPlugin;
use plugin_score::ScorePlugin;
use plugin_star::StarPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            EnemyPlugin,
            StarPlugin,
            ScorePlugin,
        ))
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (exit_game, handle_game_over))
        .run();
}
