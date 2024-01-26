use bevy::prelude::*;

use crate::{
    game_over_screen::{
        components::{GameOverScreen, RestartGameButton},
        styles::GAME_OVER_SCREEN_STYLE,
    },
    shared::styles::{
        get_button_text_style, get_title_text_style, BUTTON_STYLE, NORMAL_BUTTON_COLOR,
    },
};

pub fn spawn_game_over_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _screen_entity = build_screen(&mut commands, &asset_server);
}

pub fn despawn_game_over_screen(
    mut commands: Commands,
    game_over_screen_query: Query<Entity, With<GameOverScreen>>,
) {
    if let Ok(game_over_screen_entity) = game_over_screen_query.get_single() {
        commands.entity(game_over_screen_entity).despawn_recursive();
    }
}

fn build_screen(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let screen_entity: Entity = commands
        .spawn((
            NodeBundle {
                style: GAME_OVER_SCREEN_STYLE,
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            GameOverScreen,
        ))
        .with_children(|parent| {
            // == Game Over screen title text ==
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Game Over!",
                        get_title_text_style(asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
            // == Restart button ==
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    RestartGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Continue",
                                get_button_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();
    screen_entity
}
