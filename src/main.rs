use bevy::app::AppExit;
use bevy::audio::PlaybackMode;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0; //player sprite size in pixels
pub const PLAYER_SAFE_ZONE: f32 = PLAYER_SIZE + 10.0; //prevent enemies from spawning too close
pub const PLAYER_SPEED: f32 = 500.0;
pub const NUM_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; //enemy sprite size in pixels
pub const NUM_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; //star size in pixels
pub const STAR_SPAWN_TIME: f32 = 1.0; //seconds between additional stars spawning
pub const ENEMY_SPAWN_TIMER: f32 = 15.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_systems(
            Startup,
            (spawn_player, spawn_camera, spawn_enemies, spawn_stars),
        )
        .add_systems(
            Update,
            (
                player_movement,
                confine_player_movement,
                enemy_movement,
                update_enemy_direction,
                confine_enemy_movement,
                enemy_hit_player,
                player_hit_star,
                update_score,
                tick_timers,
                spawn_stars_over_time,
                spawn_enemies_over_time,
                exit_game,
                handle_game_over,
                update_highscores,
                highscores_updated,
            ),
        )
        .run();
}

//COMPONENTS

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star;

//RESOURCES

#[derive(Resource, Default)]
pub struct Score {
    value: u32,
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIMER, TimerMode::Repeating),
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

//EVENTS

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}

//SYSTEMS

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
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

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUM_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    //Since the Player may or may not exist
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
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
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_max = window.width() - half_player_size;
        let x_min = half_player_size;
        let y_max = window.height() - half_player_size;
        let y_min = half_player_size;
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

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
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
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut game_over_event_writer: EventWriter<GameOver>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
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
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter_mut() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let treshhold = PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0;
            if distance <= treshhold {
                println!("Player touched star, increase score!");
                score.value += 1;
                commands.entity(star_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/laser2.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..default()
                    },
                });
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score changed to {}!", score.value);
    }
}

pub fn tick_timers(
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    mut enemy_spawm_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    star_spawn_timer.timer.tick(time.delta());
    enemy_spawm_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_query: Query<Entity, With<Star>>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    let star_count = star_query.iter().len();
    if star_count < NUM_STARS && star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    enemy_spawm_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawm_timer.timer.finished() {
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

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        println!("Final score: {}", event.score);
    }
}

pub fn update_highscores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut highscores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.read() {
        highscores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn highscores_updated(highscores: Res<HighScores>) {
    if highscores.is_changed() {
        println!("Highscores: {:?}", highscores);
    }
}
