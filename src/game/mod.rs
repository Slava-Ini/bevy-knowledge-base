use bevy::prelude::*;

use crate::AppState;

use enemy::EnemyPlugin;
use events::GameOver;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::{exit_game, handle_game_over, spawn_camera};

use self::systems::toggle_simulation;

pub mod enemy;
pub mod events;
pub mod player;
pub mod score;
pub mod star;
pub mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_state::<SimulationState>()
            .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    exit_game,
                    handle_game_over,
                    toggle_simulation.run_if(in_state(AppState::Game)),
                ),
            );
    }
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
