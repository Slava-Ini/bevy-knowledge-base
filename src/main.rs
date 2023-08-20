use bevy::prelude::*;

use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;
use systems::{transition_to_game_state, transition_to_main_menu_state};

mod game;
mod main_menu;
mod systems;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin, MainMenuPlugin))
        .add_state::<AppState>()
        .add_systems(
            Update,
            (transition_to_game_state, transition_to_main_menu_state),
        )
        .run();
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
