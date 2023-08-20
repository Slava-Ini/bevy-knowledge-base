use bevy::prelude::*;

use systems::*;

mod components;
mod systems;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct ConfinementSystemSet;

// --- Note ---
// Optionally we can use enum variants and pass them insted of struct names
// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// struct PlayerSystemSet {
//     Movement,
//     Confinement,
// }

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // --- Note ---
        // We can also use `SystemSet` to group systems
        // and schedule by sets
        // For details see https://bevyengine.org/learn/migration-guides/0.10-0.11/#schedule-first-the-new-and-improved-add-systems
        app.configure_set(Update, ConfinementSystemSet.after(MovementSystemSet))
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    player_movement.in_set(MovementSystemSet),
                    confine_player_movement.in_set(ConfinementSystemSet),
                    // --- Note ---
                    // Optionally `chained` can be used or `.before`/`.after`
                    // without any `SystemSet` configuration
                    // confine_player_movement.after(player_movement),
                    enemy_hit_player,
                    player_hit_star,
                ),
            );
    }
}
