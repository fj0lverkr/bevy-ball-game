use bevy::prelude::*;
pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::GameState;

use super::SimulationState;

pub struct EnemyPlugin;

#[derive(SystemSet, Hash, Clone, Debug, Eq, PartialEq)]
pub struct MovementSystemSet;

#[derive(SystemSet, Hash, Clone, Debug, Eq, PartialEq)]
pub struct MovementConfinementSystemSet;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .configure_sets(
                Update,
                MovementSystemSet.before(MovementConfinementSystemSet),
            )
            .add_systems(OnEnter(GameState::GameRunning), spawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement.in_set(MovementSystemSet),
                    update_enemy_direction,
                    confine_enemy_movement.in_set(MovementConfinementSystemSet),
                    enemy_hit_player,
                    enemy_hit_enemy,
                    enemy_hit_star,
                    spawn_enemies_over_time,
                )
                    .run_if(in_state(GameState::GameRunning))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(GameState::GameRunning), despawn_enemies);
    }
}
