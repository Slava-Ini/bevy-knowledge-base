use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const NUMBER_OF_ENEMIES: usize = 4;

pub(crate) fn player_inputs_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player, spawn_enemies))
        .add_systems(Update, (player_movement, confine_player_movement))
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 100.0),
            texture: assets_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player,
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let x = rand::random::<f32>() * window.width();
        let y = rand::random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 50.0),
                texture: assets_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy,
        ));
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 100.0),
        ..default()
    });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.0;

        let min_x = half_player_size;
        let max_x = window.width() - half_player_size;
        let min_y = half_player_size;
        let max_y = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < min_x {
            translation.x = min_x;
        } else if translation.x > max_x {
            translation.x = max_x;
        }

        // Bound the player y position
        if translation.y < min_y {
            translation.y = min_y;
        } else if translation.y > max_y {
            translation.y = max_y;
        }

        player_transform.translation = translation;
    }
}
