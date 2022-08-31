use bevy::text::TextStyle;

use crate::widget::button::ButtonTheme;

// This is a stopgap solution until a more complete and sensible theming-system can be figured out
pub struct WidgetTheme {
    // Whatever fonts we need
    pub h1: TextStyle,
    pub p: TextStyle,
    pub icon: TextStyle,
    pub widget: TextStyle,
    // Whatever colors we need
    pub button_theme: ButtonTheme,
}
