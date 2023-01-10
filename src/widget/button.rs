use bevy::{
    ecs::query::WorldQuery,
    prelude::{
        default, info, AnyOf, Bundle, Button, Changed, Children, Color, Component, Entity,
        EventReader, EventWriter, NodeBundle, Or, Query, With,
    },
    text::Text,
    ui::{Interaction, Style, BackgroundColor},
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
#[derive(Component, Default, Clone, Copy, Debug, PartialEq, Eq)]
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
    pub disabled: Color,
    pub default: Color,
}

#[derive(Component, Clone, Debug, Default)]
pub struct ButtonTheme {
    pub background: ButtonColor,
    pub foreground: ButtonColor,
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
                background_color: theme.background.default.into(),
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

#[derive(WorldQuery)]
pub(crate) struct ButtonStateChanged {
    with: With<ButtonWidget>,
    changed: Changed<ButtonState>,
}

#[derive(WorldQuery)]
pub(crate) struct ButtonInteractionChanged {
    or: Or<(Changed<Interaction>, Changed<PrevInteraction>)>,
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub(crate) struct ButtonInteractionQuery<'a> {
    pub new: &'a Interaction,
    pub prev: &'a mut PrevInteraction,
    pub state: &'a mut ButtonState,
}

#[derive(WorldQuery)]
pub(crate) struct ButtonChanged {
    with: With<ButtonWidget>,
    changed: AnyOf<(
        Changed<ButtonState>,
        Changed<ButtonEnabledState>,
        Changed<ButtonTheme>,
    )>,
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub(crate) struct ButtonColorQuery<'a> {
    pub color: &'a mut BackgroundColor,
    pub state: &'a ButtonState,
    pub enabled: &'a ButtonEnabledState,
    pub theme: &'a ButtonTheme,
    pub children: Option<&'a Children>,
}

/// Responsible for triggering the button event if the player interacts with the button
pub(crate) fn button_trigger(
    q: Query<(Entity, &ButtonState, &TriggerPolicy, &ButtonEnabledState), ButtonStateChanged>,
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
    mut q: Query<ButtonColorQuery, ButtonStateChanged>,
    mut content: Query<&mut Text>,
) {
    for mut button_color in &mut q {
        if *button_color.enabled == ButtonEnabledState::Enabled {
            button_color.color.0 = match *button_color.state {
                ButtonState::Pressed => button_color.theme.background.pressed,
                ButtonState::Released => button_color.theme.background.released,
                ButtonState::Hovered => button_color.theme.background.hovered,
                ButtonState::None => button_color.theme.background.default,
            };
        } else {
            button_color.color.0 = button_color.theme.background.disabled;
        }

        if button_color.children.is_none() {
            continue;
        }

        for child in button_color.children.unwrap() {
            if let Ok(mut text) = content.get_mut(*child) {
                for section in text.sections.iter_mut() {
                    if *button_color.enabled == ButtonEnabledState::Enabled {
                        section.style.color = match *button_color.state {
                            ButtonState::Pressed => button_color.theme.foreground.pressed,
                            ButtonState::Released => button_color.theme.foreground.released,
                            ButtonState::Hovered => button_color.theme.foreground.hovered,
                            ButtonState::None => button_color.theme.foreground.default,
                        }
                    } else {
                        section.style.color = button_color.theme.foreground.disabled;
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
    mut changed: Query<ButtonInteractionQuery, ButtonInteractionChanged>,
) {
    for mut button in &mut changed {
        if button.prev.0 == Interaction::Clicked && *button.new == Interaction::Hovered {
            *button.state = ButtonState::Released;
        } else if button.prev.0 == Interaction::Hovered && *button.new == Interaction::Hovered {
            *button.state = ButtonState::Hovered;
        } else {
            *button.state = match button.new {
                Interaction::Clicked => ButtonState::Pressed,
                Interaction::Hovered => ButtonState::Hovered,
                Interaction::None => ButtonState::None,
            }
        }

        // Cache the state for next frame
        button.prev.0 = *button.new
    }
}
