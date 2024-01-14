use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const MAIN_MENU_STYLE: Style = {
    let mut s = Style::DEFAULT;
    s.width = Val::Percent(100.0);
    s.height = Val::Percent(100.0);
    s.flex_direction = FlexDirection::Column;
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    s.column_gap = Val::Px(8.0);
    s.row_gap = Val::Px(8.0);
    s
};

pub const BUTTON_STYLE: Style = {
    let mut s = Style::DEFAULT;
    s.width = Val::Px(200.0);
    s.height = Val::Px(80.0);
    s.align_items = AlignItems::Center;
    s.justify_content = JustifyContent::Center;
    s
};

pub const IMAGE_STYLE: Style = {
    let mut s = Style::DEFAULT;
    s.width = Val::Px(64.0);
    s.height = Val::Px(64.0);
    s.margin = UiRect {
        left: Val::Px(8.0),
        right: Val::Px(8.0),
        top: Val::Px(8.0),
        bottom: Val::Px(8.0),
    };
    s
};

pub const TITLE_STYLE: Style = {
    let mut s = Style::DEFAULT;
    s.flex_direction = FlexDirection::Row;
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    s.width = Val::Px(300.0);
    s.height = Val::Px(120.0);
    s
};

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::WHITE,
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}
