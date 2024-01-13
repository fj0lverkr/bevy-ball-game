use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if **simulation_state == SimulationState::Running {
            next_simulation_state.set(SimulationState::Paused);
            println!("Game Paused");
        } else {
            next_simulation_state.set(SimulationState::Running);
            println!("Game Running");
        }
    }
}
