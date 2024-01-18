use bevy::prelude::*;

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
