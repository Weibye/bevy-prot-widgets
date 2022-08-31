use bevy::{
    prelude::{
        Bundle, Button, Component, ComputedVisibility, GlobalTransform, Transform, Visibility,
    },
    text::Text,
    ui::{CalculatedSize, FocusPolicy, Interaction, Node, Style},
};
use material_icons::Icon;

// Resources

// State

/// Stores the state of a toggled widget element.
#[derive(Component, Clone, Debug, Default)]
pub struct ToggleState(pub bool);
