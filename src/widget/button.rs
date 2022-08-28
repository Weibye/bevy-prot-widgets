use std::process::Child;

use bevy::{
    prelude::{
        default, info, AnyOf, Bundle, Button, Changed, Children, Color, Component, Entity,
        EventReader, EventWriter, NodeBundle, Or, Query, With,
    },
    text::Text,
    ui::{Interaction, Style, UiColor},
};

/// Event that a button has triggered
pub struct ButtonEvent(Entity);

/// Marker component for a ButtonWidget
#[derive(Component, Default, Clone, Debug)]
pub struct ButtonWidget;

/// State for user interaction with the button
#[derive(Component, Default, Clone, Copy, Debug)]
pub enum ButtonState {
    Pressed,
    Released,
    Hovered,
    #[default]
    None,
}

/// Determines if user can interact with button at all
#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub enum ButtonEnabledState {
    #[default]
    Enabled,
    Disabled,
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub enum TriggerPolicy {
    #[default]
    OnRelease,
    OnPress,
}

#[derive(Component, Clone, Debug, Default)]
pub struct PrevInteraction(Interaction);

#[derive(Debug, Clone, Copy, Default)]
pub struct ButtonColor {
    pub pressed: Color,
    pub released: Color,
    pub hovered: Color,
    pub default: Color,
}

#[derive(Component, Clone, Debug, Default)]
pub struct ButtonTheme {
    pub background_enabled: ButtonColor,
    pub background_disabled: ButtonColor,
    pub foreground_enabled: ButtonColor,
    pub foreground_disabled: ButtonColor,
}

/// A Button Widget
#[derive(Bundle, Clone, Debug, Default)]
pub struct ButtonWidgetBundle {
    #[bundle]
    pub node_bundle: NodeBundle,
    /// Enables clicking on the widget
    pub button: Button,
    /// Interaction state of the widget
    pub interaction: Interaction,
    /// Tracks the last known interaction
    pub prev_interaction: PrevInteraction,
    /// Marker component
    pub widget: ButtonWidget,
    pub state: ButtonState,
    pub policy: TriggerPolicy,
    pub enabled: ButtonEnabledState,
    pub theme: ButtonTheme,
}

impl ButtonWidgetBundle {
    pub fn new(style: Style, theme: ButtonTheme) -> Self {
        ButtonWidgetBundle {
            node_bundle: NodeBundle {
                style,
                color: UiColor(theme.background_enabled.default.into()),
                ..default()
            },
            theme,
            ..default()
        }
    }

    pub fn with_policy(mut self, policy: TriggerPolicy) -> Self {
        self.policy = policy;
        self
    }
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = if enabled {
            ButtonEnabledState::Enabled
        } else {
            ButtonEnabledState::Disabled
        };
        self
    }
    pub fn with_theme(mut self, theme: ButtonTheme) -> Self {
        self.theme = theme;
        self
    }
}

/// Responsible for triggering the button event if the player interacts with the button
pub(crate) fn button_trigger(
    q: Query<
        (Entity, &ButtonState, &TriggerPolicy, &ButtonEnabledState),
        (With<ButtonWidget>, Changed<ButtonState>),
    >,
    mut trigger: EventWriter<ButtonEvent>,
) {
    for (button, state, policy, enabled) in &q {
        // Should not be allowed to trigger a button that is disabled
        if *enabled == ButtonEnabledState::Disabled {
            continue;
        }

        match (*state, *policy) {
            (ButtonState::Pressed, TriggerPolicy::OnPress) => {
                trigger.send(ButtonEvent(button));
            }
            (ButtonState::Released, TriggerPolicy::OnRelease) => {
                trigger.send(ButtonEvent(button));
            }
            _ => {}
        }
    }
}

pub(crate) fn button_color(
    mut q: Query<
        (
            &mut UiColor,
            &ButtonState,
            &ButtonEnabledState,
            &ButtonTheme,
            Option<&Children>,
        ),
        (
            With<ButtonWidget>,
            AnyOf<(
                Changed<ButtonState>,
                Changed<ButtonEnabledState>,
                Changed<ButtonTheme>,
            )>,
        ),
    >,
    mut content: Query<&mut Text>,
) {
    for (mut color, state, enabled, button_theme, children) in &mut q {
        let (background_theme, foreground_theme) = match enabled {
            ButtonEnabledState::Enabled => (
                &button_theme.background_enabled,
                &button_theme.foreground_enabled,
            ),
            ButtonEnabledState::Disabled => (
                &button_theme.background_disabled,
                &button_theme.foreground_disabled,
            ),
        };

        color.0 = match *state {
            ButtonState::Pressed => background_theme.pressed,
            ButtonState::Released => background_theme.released,
            ButtonState::Hovered => background_theme.hovered,
            ButtonState::None => background_theme.default,
        };

        if children.is_none() {
            continue;
        }

        for child in children.unwrap() {
            if let Ok(mut text) = content.get_mut(*child) {
                for section in text.sections.iter_mut() {
                    section.style.color = match *state {
                        ButtonState::Pressed => foreground_theme.pressed,
                        ButtonState::Released => foreground_theme.released,
                        ButtonState::Hovered => foreground_theme.hovered,
                        ButtonState::None => foreground_theme.default,
                    }
                }
            }
        }
    }
}

pub(crate) fn on_button_trigger(mut reader: EventReader<ButtonEvent>) {
    for event in &mut reader.iter() {
        // TOOD: Figure out how to ergonomically assign specific actions to specific game-play elements
        info!("Button was triggered: {:?}", event.0);
    }
}

// TODO: This need to clear the pressed state the next frame
pub(crate) fn button_interaction(
    mut changed: Query<
        (&Interaction, &mut PrevInteraction, &mut ButtonState),
        Or<(Changed<Interaction>, Changed<PrevInteraction>)>,
    >,
) {
    for (new_state, mut prev_state, mut button_state) in &mut changed {
        if prev_state.0 == Interaction::Clicked && *new_state == Interaction::Hovered {
            *button_state = ButtonState::Released;
        } else if prev_state.0 == Interaction::Hovered && *new_state == Interaction::Hovered {
            *button_state = ButtonState::Hovered;
        } else {
            *button_state = match new_state {
                Interaction::Clicked => ButtonState::Pressed,
                Interaction::Hovered => ButtonState::Hovered,
                Interaction::None => ButtonState::None,
            }
        }

        // Cache the state for next frame
        prev_state.0 = *new_state
    }
}
