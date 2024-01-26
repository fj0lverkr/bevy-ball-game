use bevy::{app::AppExit, prelude::*};

use crate::{
    game::SimulationState,
    game_over_screen::components::{ExitGameButton, RestartGameButton},
    shared::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
    GameState,
};

pub fn interact_with_restart_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RestartGameButton>),
    >,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                game_state_next_state.set(GameState::GameRunning);
                simulation_state_next_state.set(SimulationState::Paused);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            _ => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ExitGameButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            _ => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
