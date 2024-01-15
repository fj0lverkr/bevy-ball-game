use bevy::prelude::*;

use super::components::{EnemyCounter, StarCounter};
use crate::game::{plugin_enemy::components::Enemy, plugin_score::resources::Score};

pub mod layout;

pub fn update_number_of_enemies(
    mut text_field_query: Query<&mut Text, With<EnemyCounter>>,
    enemies_query: Query<Entity, With<Enemy>>,
) {
    if let Ok(mut text_field) = text_field_query.get_single_mut() {
        let num_enemies = enemies_query.iter().count();
        text_field.sections[0].value = num_enemies.to_string();
    }
}

pub fn update_score_counter(
    mut text_field_query: Query<&mut Text, With<StarCounter>>,
    score: Res<Score>,
) {
    if score.is_changed() {
        if let Ok(mut text_field) = text_field_query.get_single_mut() {
            text_field.sections[0].value = score.value.to_string();
        }
    }
}
