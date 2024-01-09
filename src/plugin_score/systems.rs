use crate::{plugin_gameloop::events::GameOver, plugin_score::resources::Score};
use bevy::prelude::*;

use super::resources::HighScores;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score changed to {}!", score.value);
    }
}

pub fn update_highscores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut highscores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.read() {
        highscores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn highscores_updated(highscores: Res<HighScores>) {
    if highscores.is_changed() {
        println!("Highscores: {:?}", highscores);
    }
}
