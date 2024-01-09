use bevy::prelude::*;
mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, spawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    enemy_hit_player,
                    spawn_enemies_over_time,
                ),
            );
    }
}
