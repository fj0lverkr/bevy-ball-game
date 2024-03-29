use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::GameState;

use super::events::GameOver;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for _event in game_over_event_reader.read() {
        next_game_state.set(GameState::GameOver);
    }
}

pub fn handle_esc_pressed(
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if **game_state == GameState::GameRunning {
            next_game_state.set(GameState::MainMenu);
        } else {
            app_exit_event_writer.send(AppExit);
        }
    }
}
