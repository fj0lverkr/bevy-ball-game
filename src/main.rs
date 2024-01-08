mod events;
mod plugin_enemy;
mod plugin_player;
mod plugin_score;
mod plugin_star;
mod systems;

use events::*;
use plugin_score::resources::{HighScores, Score};
use systems::*;

use plugin_enemy::EnemyPlugin;
use plugin_player::PlayerPlugin;
use plugin_score::systems::*;
use plugin_star::StarPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, EnemyPlugin, StarPlugin))
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (
                update_score,
                exit_game,
                handle_game_over,
                update_highscores,
                highscores_updated,
            ),
        )
        .run();
}
