use bevy::prelude::*;

pub const HUD_STYLE: Style = {
    let mut s = Style::DEFAULT;
    s.width = Val::Percent(100.0);
    s.height = Val::Percent(10.0);
    s.display = Display::Flex;
    s.flex_direction = FlexDirection::Row;
    s.justify_content = JustifyContent::SpaceBetween;
    s.align_content = AlignContent::SpaceBetween;
    s.padding = UiRect {
        left: Val::Px(20.0),
        right: Val::Px(20.0),
        top: Val::Px(10.0),
        bottom: Val::Px(0.0),
    };
    s
};

pub const HUD_ITEM_STYLE: Style = {
    let mut s = Style::DEFAULT;
    s.display = Display::Flex;
    s.flex_direction = FlexDirection::Row;
    s.align_items = AlignItems::Center;
    s.justify_content = JustifyContent::Center;
    s.width = Val::Auto;
    s.height = Val::Auto;
    s.padding = UiRect {
        left: Val::Px(15.0),
        right: Val::Px(15.0),
        top: Val::Px(5.0),
        bottom: Val::Px(5.0),
    };
    s.border = UiRect {
        left: Val::Px(2.0),
        right: Val::Px(2.0),
        top: Val::Px(2.0),
        bottom: Val::Px(2.0),
    };
    s
};

pub const HUD_ITEM_BORDER_COLOR: Color = Color::rgb(53.0, 53.0, 53.0);

pub const IMAGE_STYLE: Style = {
    let mut s = Style::DEFAULT;
    s.width = Val::Px(32.0);
    s.height = Val::Px(32.0);
    s.margin = UiRect {
        left: Val::Px(2.0),
        right: Val::Px(10.0),
        top: Val::Px(2.0),
        bottom: Val::Px(2.0),
    };
    s
};

pub fn get_hud_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}
