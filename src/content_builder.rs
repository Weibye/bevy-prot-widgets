use bevy_hierarchy::ChildBuilder;
use bevy_ui::{prelude::TextBundle, Style, UiRect, Val};

use crate::theme::WidgetTheme;

/// Creates a new H1 title
pub fn create_h1(container: &mut ChildBuilder, theme: &WidgetTheme, text: &str) {
    container.spawn(
        TextBundle::from_section(text, theme.h1.clone()).with_style(Style {
            margin: UiRect::new(
                Val::Undefined,
                Val::Undefined,
                Val::Px(15.0),
                Val::Undefined,
            ),
            ..Default::default()
        }),
    );
}

/// Create a new paragraph section
pub fn create_p(container: &mut ChildBuilder, theme: &WidgetTheme, text: &str) {
    container.spawn(TextBundle::from_section(text, theme.p.clone()));
}
