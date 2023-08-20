use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use super::resources::*;
use super::{NUMBER_OF_STARS, STAR_SIZE};

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let x = rand::random::<f32>() * (window.width() - STAR_SIZE) + STAR_SIZE / 2.0;
        let y = rand::random::<f32>() * (window.height() - STAR_SIZE) + STAR_SIZE / 2.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: assets_server.load("sprites/star.png"),
                ..default()
            },
            Star,
        ));
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_timer(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = rand::random::<f32>() * window.width();
        let random_y = rand::random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star,
        ));
    }
}
