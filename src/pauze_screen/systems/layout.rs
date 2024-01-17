use bevy::prelude::*;

use crate::pauze_screen::components::PauzeScreen;

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

fn build_pauze_screen(commands: &mut Commands, asset_server: &AssetServer) -> Entity {
    let pauze_screen_entity: Entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                ..default()
            },
            PauzeScreen,
        ))
        .id();
    pauze_screen_entity
}
