mod events;
mod plugin_enemy;
mod plugin_player;
mod plugin_score;
mod plugin_star;
mod systems;

use events::*;
use plugin_score::resources::{HighScores, Score};
use systems::*;

use plugin_enemy::{components::Enemy, resources::EnemySpawnTimer, systems::*};
use plugin_player::{components::Player, systems::*};
use plugin_score::systems::*;
use plugin_star::{components::Star, resources::StarSpawnTimer, systems::*};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_systems(
            Startup,
            (spawn_player, spawn_camera, spawn_enemies, spawn_stars),
        )
        .add_systems(
            Update,
            (
                player_movement,
                confine_player_movement,
                enemy_movement,
                update_enemy_direction,
                confine_enemy_movement,
                enemy_hit_player,
                player_hit_star,
                update_score,
                spawn_stars_over_time,
                spawn_enemies_over_time,
                exit_game,
                handle_game_over,
                update_highscores,
                highscores_updated,
            ),
        )
        .run();
}
