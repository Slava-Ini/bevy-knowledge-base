use bevy::audio::{Volume, VolumeLevel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use super::resources::*;
use super::{ENEMY_SIZE, ENEMY_SPEED, NUMBER_OF_ENEMIES};

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let x = rand::random::<f32>() * (window.width() - ENEMY_SIZE) + ENEMY_SIZE / 2.0;
        let y = rand::random::<f32>() * (window.height() - ENEMY_SIZE) + ENEMY_SIZE / 2.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(rand::random::<f32>(), rand::random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &mut Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;

    let min_x = half_enemy_size;
    let max_x = window.width() - half_enemy_size;
    let min_y = half_enemy_size;
    let max_y = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        let translation = transform.translation;

        if translation.x < min_x {
            enemy.direction.x = enemy.direction.x.abs();
            direction_changed = true;
        }
        if translation.x > max_x {
            enemy.direction.x = -1.0 * enemy.direction.x.abs();
            direction_changed = true;
        }
        if translation.y < min_y {
            enemy.direction.y = enemy.direction.y.abs();
            direction_changed = true;
        }
        if translation.y > max_y {
            enemy.direction.y = -1.0 * enemy.direction.y.abs();
            direction_changed = true;
        }

        if direction_changed {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

            let sound_effect = if rand::random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };

            let playback_settings = PlaybackSettings {
                volume: Volume::Relative(VolumeLevel::new(0.2)),
                ..default()
            };

            commands.spawn(AudioBundle {
                source: sound_effect,
                settings: playback_settings,
                ..default()
            });
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut enemy_transform) = enemy_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_enemy_size = ENEMY_SIZE / 2.0;

        let min_x = half_enemy_size;
        let max_x = window.width() - half_enemy_size;
        let min_y = half_enemy_size;
        let max_y = window.height() - half_enemy_size;

        let mut translation = enemy_transform.translation;

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

        enemy_transform.translation = translation;
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    enemy_spawn_timer.timer.tick(time.delta());

    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = rand::random::<f32>() * window.width();
        let random_y = rand::random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(1.0, 1.0),
            },
        ));
    }
}
