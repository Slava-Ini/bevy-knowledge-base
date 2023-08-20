use bevy::prelude::*;

use crate::enemy::EnemyPlugin;
use crate::events::GameOver;
use crate::player::PlayerPlugin;
use crate::score::ScorePlugin;
use crate::star::StarPlugin;
use crate::systems::exit_game;
use systems::*;

mod enemy;
mod events;
mod player;
mod score;
mod star;
mod systems;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EnemyPlugin,
            PlayerPlugin,
            ScorePlugin,
            StarPlugin,
        ))
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (exit_game, handle_game_over))
        .run();
}
