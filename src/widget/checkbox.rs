use bevy::{
    prelude::{
        Bundle, Button, Component, ComputedVisibility, GlobalTransform, Transform, Visibility,
    },
    text::Text,
    ui::{CalculatedSize, FocusPolicy, Interaction, Node, Style, UiColor, UiImage},
};
use material_icons::Icon;

use crate::ToggleState;

/// Marker component for a [CheckboxWidget].
#[derive(Component, Debug, Clone, Default)]
pub struct CheckboxWidget;

#[derive(Component, Debug, Clone, Default)]
pub enum CheckboxState {
    Checked,
    #[default]
    Unchecked,
    Indeterminate,
}

/// Component that defines the icons of the [CheckboxWidget].
#[derive(Component, Debug, Clone)]
pub struct CheckboxIcons {
    pub unchecked: Icon,
    pub checked: Icon,
    pub indeterminate: Icon,
}

impl Default for CheckboxIcons {
    fn default() -> Self {
        Self {
            unchecked: Icon::CheckBoxOutlineBlank,
            checked: Icon::CheckBox,
            indeterminate: Icon::IndeterminateCheckBox,
        }
    }
}

/// A Checkbox Widget
#[derive(Bundle, Clone, Debug, Default)]
pub struct CheckboxBundle {
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
    pub checkbox: CheckboxWidget,
    /// State of the widget
    pub toggle: ToggleState,
    /// The different icons for the widget
    pub icons: CheckboxIcons,
    /// The different state the checkbox can be in
    pub state: CheckboxState,
    /// Background color
    pub color: UiColor,
    /// Describes the image of the node
    pub image: UiImage,
}
