use bevy::prelude::*;

use crate::game::score::resources::HighScores;
use crate::game::score::systems::*;
use crate::AppState;

pub mod resources;
pub mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), insert_score)
            .init_resource::<HighScores>()
            .add_systems(
                Update,
                (
                    update_score.run_if(in_state(AppState::Game)),
                    update_high_scores,
                    high_scores_update,
                ),
            )
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}
