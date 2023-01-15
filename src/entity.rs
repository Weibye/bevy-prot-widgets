// Resources

// State

use bevy_ecs::prelude::Component;

/// Stores the state of a toggled widget element.
#[derive(Component, Clone, Debug, Default)]
pub struct ToggleState(pub bool);
