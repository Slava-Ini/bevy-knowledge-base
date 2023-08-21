use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::SimulationState;

use crate::game::GameOver;
use crate::AppState;

pub fn pause_simulation(mut simulation_next_state: ResMut<NextState<SimulationState>>) {
    simulation_next_state.set(SimulationState::Paused);
    println!("Pausing simulation");
}

pub fn resume_simulation(mut simulation_next_state: ResMut<NextState<SimulationState>>) {
    simulation_next_state.set(SimulationState::Running);
    println!("Resuming simulation");
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_app_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.get() {
            SimulationState::Running => {
                next_app_state.set(SimulationState::Paused);
                println!("Pausing simulation");
            }
            SimulationState::Paused => {
                next_app_state.set(SimulationState::Running);
                println!("Resuming simulation");
            }
        }
    }
}

// --- Note ---
// Below systems might as well be in the root `systems.rs`
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 100.0),
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

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.iter() {
        println!("Game Over! Final Score: {}", event.score);
        next_app_state.set(AppState::GameOver);
    }
}
