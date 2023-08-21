use bevy::prelude::*;

use crate::game::SimulationState;
use crate::AppState;

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.get() != &AppState::Game {
            // --- Note ---
            // Don't do this
            // commands.insert_resource(NextState(Some(AppState::Game)));
            next_app_state.set(AppState::Game);
            println!("Transitioning to Game state");
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.get() != &AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            println!("Transitioning to Main Menu state");
        }
    }
}
