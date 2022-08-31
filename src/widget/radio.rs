use bevy::{
    prelude::{
        Bundle, Button, Component, ComputedVisibility, GlobalTransform, Transform, Visibility,
    },
    text::Text,
    ui::{CalculatedSize, FocusPolicy, Interaction, Node, Style},
};
use material_icons::Icon;

use crate::entity::ToggleState;

/// Marker component for a CheckBoxWidget
#[derive(Component, Debug, Clone, Default)]
pub struct RadioButtonWidget;

/// Component that defines the icons of the [CheckboxWidget].
#[derive(Component, Debug, Clone)]
pub struct RadioButtonIcons {
    pub empty: Icon,
    pub checked: Icon,
}

impl Default for RadioButtonIcons {
    fn default() -> Self {
        Self {
            empty: Icon::RadioButtonUnchecked,
            checked: Icon::RadioButtonChecked,
        }
    }
}

/// A Checkbox Widget
#[derive(Bundle, Clone, Debug, Default)]
pub struct RadioButtonBundle {
    // From image-bundle
    /// Describes the size of the node
    pub node: Node,
    /// Describes the style including flexbox settings
    pub style: Style,
    /// Contains the text of the node
    pub text: Text,
    /// The calculated size based on the given image
    pub calculated_size: CalculatedSize,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    pub transform: Transform,
    /// The global transform of the node
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,

    // Additional stuff
    /// Enables clicking on the widget
    pub button: Button,
    /// Interaction state of the widget
    pub interaction: Interaction,
    /// Marker to make it a "CheckboxWidget"
    pub radio: RadioButtonWidget,
    /// State of the widget
    pub toggle: ToggleState,
    /// The different icons for the widget
    pub icons: RadioButtonIcons,
}
