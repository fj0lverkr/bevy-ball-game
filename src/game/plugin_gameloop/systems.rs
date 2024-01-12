use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::{game::SimulationState, GameState};

use super::events::GameOver;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn handle_game_over(mut commands: Commands, mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        commands.insert_resource(NextState(Some(GameState::GameOver)));
        println!("Final score: {}", event.score);
    }
}

pub fn transition_to_running_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
) {
    if (keyboard_input.just_pressed(KeyCode::Return)
        || keyboard_input.just_pressed(KeyCode::NumpadEnter))
        && **game_state != GameState::GameRunning
    {
        commands.insert_resource(NextState(Some(GameState::GameRunning)));
        commands.insert_resource(NextState(Some(SimulationState::Running)));
        println!("transitioned to GameRunning state.");
    }
}

pub fn handle_esc_pressed(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if **game_state == GameState::GameRunning {
            commands.insert_resource(NextState(Some(GameState::MainMenu)));
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("transitioned to MainMenu state")
        } else if **game_state == GameState::MainMenu {
            commands.insert_resource(NextState(Some(GameState::GameRunning)));
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("transitioned to GameRunning state.");
        } else {
            app_exit_event_writer.send(AppExit);
        }
    }
}
