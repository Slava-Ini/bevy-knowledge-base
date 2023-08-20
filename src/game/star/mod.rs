use bevy::prelude::*;

use crate::game::SimulationState;
use crate::AppState;
use resources::*;
use systems::*;

pub mod components;
pub mod resources;
pub mod systems;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            // .add_systems(Startup, spawn_stars)
            // .add_systems(Update, (tick_star_spawn_timer, spawn_stars_over_timer));
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_timer)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_stars);
    }
}
