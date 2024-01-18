use bevy::prelude::*;

use crate::{
    pauze_screen::{
        components::{ContinueButton, PauzeScreen},
        styles::PAUZE_SCREEN_STYLE,
    },
    shared::styles::{
        get_button_text_style, get_title_text_style, BUTTON_STYLE, NORMAL_BUTTON_COLOR,
    },
};

pub fn spawn_pauze_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _pauze_screen_entity = build_pauze_screen(&mut commands, &asset_server);
}

pub fn despawn_pauze_screen(
    mut commands: Commands,
    pauze_screen_query: Query<Entity, With<PauzeScreen>>,
) {
    if let Ok(pauze_screen_entity) = pauze_screen_query.get_single() {
        commands.entity(pauze_screen_entity).despawn_recursive();
    }
}

fn build_pauze_screen(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pauze_screen_entity: Entity = commands
        .spawn((
            NodeBundle {
                style: PAUZE_SCREEN_STYLE,
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            PauzeScreen,
        ))
        .with_children(|parent| {
            // == Pauze screen title text ==
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Game Paused",
                        get_title_text_style(asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
            // == Play Button ==
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    ContinueButton,
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
    pauze_screen_entity
}
