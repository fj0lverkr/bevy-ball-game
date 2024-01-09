use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub const PLAYER_SIZE: f32 = 64.0; //player sprite size in pixels
pub const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

#[derive(SystemSet, Hash, Clone, Debug, Eq, PartialEq)]
pub struct MovementSystemSet;

#[derive(SystemSet, Hash, Clone, Debug, Eq, PartialEq)]
pub struct MovementConfinementSystemSet;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            MovementSystemSet.before(MovementConfinementSystemSet),
        )
        .add_systems(Startup, spawn_player)
        .add_systems(
            Update,
            (
                player_movement.in_set(MovementSystemSet),
                confine_player_movement.in_set(MovementConfinementSystemSet),
                player_hit_star,
            ),
        );
    }
}
