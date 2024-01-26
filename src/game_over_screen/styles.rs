use bevy::prelude::*;

pub const GAME_OVER_SCREEN_STYLE: Style = {
    let mut s = Style::DEFAULT;
    s.width = Val::Percent(100.0);
    s.height = Val::Percent(100.0);
    s.display = Display::Flex;
    s.flex_direction = FlexDirection::Column;
    s.align_items = AlignItems::Center;
    s.justify_content = JustifyContent::Center;
    s.column_gap = Val::Px(8.0);
    s.row_gap = Val::Px(8.0);
    s
};
