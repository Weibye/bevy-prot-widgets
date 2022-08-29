use bevy::{prelude::{ChildBuilder, default, BuildChildren, TextBundle}, ui::{Size, UiRect, Style, Val, AlignItems, JustifyContent, FocusPolicy}};
use material_icons::Icon;

use crate::{widget::button::{ButtonWidgetBundle, TriggerPolicy}, theme::WidgetTheme};

/// Creates a new H1 title
pub fn create_h1(container: &mut ChildBuilder, theme: &WidgetTheme, text: &str) {
    container.spawn_bundle(TextBundle::from_section(text, theme.h1.clone())
    .with_style(Style {
        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(15.0), Val::Undefined),
       ..default()
   }));
}

/// Create a new paragraph section
pub fn create_p(container: &mut ChildBuilder, theme: &WidgetTheme, text: &str) {
    container.spawn_bundle(TextBundle::from_section(text, theme.p.clone()));
}

/// Creates a new Text Button Widget
pub fn create_text_button(container: &mut ChildBuilder, theme: &WidgetTheme, text: &str, enabled: bool, policy: TriggerPolicy) {
    container.spawn_bundle(
        ButtonWidgetBundle::new(Style {
            size: Size::new(Val::Auto, Val::Auto),
            padding: UiRect::new(Val::Px(15.0), Val::Px(15.0), Val::Px(10.0), Val::Px(10.0)),
            margin: UiRect::all(Val::Px(4.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }, 
        theme.button_theme.clone()
    ).with_enabled(enabled).with_policy(policy)
    ).with_children(| button | {
        button.spawn_bundle(TextBundle::from_section(text,theme.widget.clone()));
    });
}

/// Creates a new icon-button widget
pub fn create_icon_button(container: &mut ChildBuilder, theme: &WidgetTheme, icon: Icon, enabled: bool, policy: TriggerPolicy) {
    container.spawn_bundle(
        ButtonWidgetBundle::new(Style {
            size: Size::new(Val::Auto, Val::Auto),
            padding: UiRect::new(Val::Px(15.0), Val::Px(15.0), Val::Px(10.0), Val::Px(10.0)),
            margin: UiRect::all(Val::Px(4.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }, 
        theme.button_theme.clone()
    ).with_enabled(enabled).with_policy(policy)
    ).with_children(| button | {
        button.spawn_bundle(TextBundle::from_section(icon.to_string(), theme.icon.clone()));
    });
}

/// Creates a new button with icon and text
pub fn create_label_button(container: &mut ChildBuilder, theme: &WidgetTheme, icon: Icon, text: &str, enabled: bool, policy: TriggerPolicy) {
    container.spawn_bundle(
        ButtonWidgetBundle::new(Style {
            size: Size::new(Val::Auto, Val::Auto),
            padding: UiRect::new(Val::Px(15.0), Val::Px(15.0), Val::Px(10.0), Val::Px(10.0)),
            margin: UiRect::all(Val::Px(4.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }, 
        theme.button_theme.clone()
    ).with_enabled(enabled).with_policy(policy)
    ).with_children(| button | {
        button.spawn_bundle(
            TextBundle::from_section(icon.to_string(), theme.icon.clone())
                .with_style(Style {
                    margin: UiRect::new(Val::Undefined, Val::Px(5.0), Val::Undefined, Val::Undefined),
                    ..default()
                })
        );
        button.spawn_bundle(TextBundle::from_section(text,theme.widget.clone()));
    });
}