use bevy::prelude::*;

pub const BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const BUTTON_STYLE: Style = {
    let mut s = Style::DEFAULT;
    s.width = Val::Px(200.0);
    s.height = Val::Px(80.0);
    s.align_items = AlignItems::Center;
    s.justify_content = JustifyContent::Center;
    s
};
