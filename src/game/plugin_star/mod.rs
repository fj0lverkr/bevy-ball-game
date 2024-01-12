use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::GameState;

use super::SimulationState;

pub const NUM_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; //star size in pixels

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(GameState::GameRunning), spawn_stars)
            .add_systems(
                Update,
                spawn_stars_over_time
                    .run_if(in_state(GameState::GameRunning))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(GameState::GameRunning), despawn_stars);
    }
}
