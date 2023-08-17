use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn bundles_and_resources_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player))
        .run();
}

#[derive(Component)]
struct Player;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.width() / 2.0, 0.0),
            texture: assets_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player,
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.width() / 2.0, 100.0),
        ..default()
    });
}
