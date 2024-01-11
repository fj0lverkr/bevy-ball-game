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

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        println!("Final score: {}", event.score);
    }
}

pub fn transition_to_running_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return)
        || keyboard_input.just_pressed(KeyCode::NumpadEnter)
            && **game_state != GameState::GameRunning
    {
        commands.insert_resource(NextState(Some(GameState::GameRunning)));
    }
}
