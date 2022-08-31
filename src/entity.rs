use bevy::prelude::Component;

// Resources

// State

/// Stores the state of a toggled widget element.
#[derive(Component, Clone, Debug, Default)]
pub struct ToggleState(pub bool);
