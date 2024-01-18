use bevy::{audio::PlaybackMode, prelude::*, window::PrimaryWindow};
use rand::random;

use crate::game::plugin_gameloop::events::GameOver;
use crate::game::plugin_player::components::Player;
use crate::game::plugin_player::PLAYER_SIZE;
use crate::game::plugin_score::resources::Score;

use super::components::Enemy;
use super::resources::EnemySpawnTimer;

pub const NUM_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; //enemy sprite size in pixels
pub const PLAYER_SAFE_ZONE: f32 = PLAYER_SIZE + 10.0; //prevent enemies from spawning too close

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if enemy_query.is_empty() {
        let window = window_query.get_single().unwrap();
        for _ in 0..NUM_ENEMIES {
            let mut random_x = random::<f32>() * window.width();
            let mut random_y = random::<f32>() * window.height();
            let player_transform =
                Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0);
            let mut enemy_transform = Transform::from_xyz(random_x, random_y, 0.0);

            while enemy_transform
                .translation
                .distance(player_transform.translation)
                < PLAYER_SAFE_ZONE
            {
                random_x = random::<f32>() * window.width();
                random_y = random::<f32>() * window.height();
                enemy_transform = Transform::from_xyz(random_x, random_y, 0.0);
            }

            commands.spawn((
                SpriteBundle {
                    transform: enemy_transform,
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
            ));
        }
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let half_size = ENEMY_SIZE / 2.0;
    let x_max = window.width() - half_size;
    let x_min = half_size;
    let y_max = window.height() - half_size;
    let y_min = half_size;
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        let translation = transform.translation;
        //bound to x
        if translation.x <= x_min || translation.x >= x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y <= y_min || translation.y >= y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }
        if direction_changed {
            let audio_path = if random::<f32>() > 0.5 {
                "audio/pluck_001.ogg"
            } else {
                "audio/pluck_002.ogg"
            };
            commands.spawn(AudioBundle {
                source: asset_server.load(audio_path),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    ..default()
                },
            });
        }
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let half_size = ENEMY_SIZE / 2.0;
    let x_max = window.width() - half_size;
    let x_min = half_size;
    let y_max = window.height() - half_size;
    let y_min = half_size;
    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        //bound to x
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        //bound to Y
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut game_over_event_writer: EventWriter<GameOver>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance <= player_radius + enemy_radius {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..default()
                    },
                });
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut enemy_spawm_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    if enemy_spawm_timer.timer.tick(time.delta()).finished() {
        let window = window_query.get_single().unwrap();
        let mut random_x = random::<f32>() * window.width();
        let mut random_y = random::<f32>() * window.height();
        let mut enemy_transform = Transform::from_xyz(random_x, random_y, 0.0);

        if let Ok(player_transform) = player_query.get_single() {
            while enemy_transform
                .translation
                .distance(player_transform.translation)
                < PLAYER_SAFE_ZONE
            {
                random_x = random::<f32>() * window.width();
                random_y = random::<f32>() * window.height();
                enemy_transform = Transform::from_xyz(random_x, random_y, 0.0);
            }
        }

        commands.spawn((
            SpriteBundle {
                transform: enemy_transform,
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn();
    }
}

pub fn enemy_hit_enemy(
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy), With<Enemy>>,
    other_enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (enemy_entity, enemy_transform, mut enemy) in enemy_query.iter_mut() {
        for (other_entity, other_transform) in other_enemy_query.iter() {
            if enemy_entity != other_entity {
                let distance = enemy_transform
                    .translation
                    .distance(other_transform.translation);
                if distance <= ENEMY_SIZE {
                    enemy.direction = -enemy.direction; //TODO: correct bouncing direction and
                                                        //prevent enemies from spawning on eachother.
                }
            }
        }
    }
}
