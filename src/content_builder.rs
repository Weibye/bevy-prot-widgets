use bevy_hierarchy::{BuildChildren, ChildBuilder};
use bevy_ui::{
    prelude::{NodeBundle, TextBundle},
    AlignItems, JustifyContent, Size, Style, UiRect, Val,
};
use material_icons::Icon;

use crate::{
    theme::WidgetTheme,
    widget::button::{ButtonEnabledState, ButtonWidgetBundle, TriggerPolicy},
};

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

/// Creates a new button with icon and text
pub fn create_label_button(
    container: &mut ChildBuilder,
    theme: &WidgetTheme,
    icon: Icon,
    text: &str,
    enabled: bool,
    policy: TriggerPolicy,
) {
    container
        .spawn(ButtonWidgetBundle {
            node_bundle: NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    padding: UiRect::new(
                        Val::Px(15.0),
                        Val::Px(15.0),
                        Val::Px(10.0),
                        Val::Px(10.0),
                    ),
                    margin: UiRect::all(Val::Px(4.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            theme: theme.button_theme.clone(),
            policy,
            enabled: if enabled {
                ButtonEnabledState::Enabled
            } else {
                ButtonEnabledState::Disabled
            },
            ..Default::default()
        })
        .with_children(|button| {
            button.spawn(
                TextBundle::from_section(icon.to_string(), theme.icon.clone()).with_style(Style {
                    margin: UiRect::new(
                        Val::Undefined,
                        Val::Px(5.0),
                        Val::Undefined,
                        Val::Undefined,
                    ),
                    ..Default::default()
                }),
            );
            button.spawn(TextBundle::from_section(text, theme.widget.clone()));
        });
}
