use bevy::prelude::*;

use crate::game::plugin_hud::{
    components::{EnemyCounter, Hud, StarCounter},
    styles::{get_hud_text_style, HUD_ITEM_BORDER_COLOR, HUD_ITEM_STYLE, HUD_STYLE, IMAGE_STYLE},
};

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _hud = build_hud(&mut commands, &asset_server);
}

pub fn despawn_hud(mut commands: Commands, hud_entity_query: Query<Entity, With<Hud>>) {
    if let Ok(hud_entity) = hud_entity_query.get_single() {
        commands.entity(hud_entity).despawn_recursive();
    }
}

fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            // -- MAIN HUD NODE --
            NodeBundle {
                style: HUD_STYLE,
                z_index: ZIndex::Global(5),
                ..default()
            },
            Hud,
        ))
        .with_children(|parent| {
            // -- LEFT HUD ITEM --
            parent
                .spawn(NodeBundle {
                    style: HUD_ITEM_STYLE,
                    border_color: HUD_ITEM_BORDER_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // --- ENEMY IMAGE ---
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_red_large.png").into(),
                        ..default()
                    });
                    // ___ NUMBER OF ENEMIES ---
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "0",
                                    get_hud_text_style(asset_server),
                                )],
                                ..default()
                            },
                            ..default()
                        },
                        EnemyCounter,
                    ));
                });
            // -- RIGHT HUD ITEM --
            parent
                .spawn(NodeBundle {
                    style: HUD_ITEM_STYLE,
                    border_color: HUD_ITEM_BORDER_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // --- SCORE IMAGE ---
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/star.png").into(),
                        ..default()
                    });
                    // ___ SCORE ---
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "0",
                                    get_hud_text_style(asset_server),
                                )],
                                ..default()
                            },
                            ..default()
                        },
                        StarCounter,
                    ));
                });
        })
        .id();
    hud_entity
}
