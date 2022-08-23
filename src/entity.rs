use bevy::{
    prelude::{
        Bundle, Button, Component, ComputedVisibility, Deref, GlobalTransform, Handle, Image,
        Transform, Visibility,
    },
    text::Text,
    ui::{
        widget::ImageMode, CalculatedSize, FocusPolicy, Interaction, Node, Style, UiColor, UiImage,
    },
};
use material_icons::Icon;

// Resources

// Widgets

/// Marker component for a [CheckboxWidget].
#[derive(Component, Debug, Clone, Default)]
pub struct CheckboxWidget;

/// Component that defines the icons of the [CheckboxWidget].
#[derive(Component, Debug, Clone)]
pub struct CheckboxIcons {
    pub empty: Icon,
    pub checked: Icon,
}

impl Default for CheckboxIcons {
    fn default() -> Self {
        Self {
            empty: Icon::CheckBoxOutlineBlank,
            checked: Icon::CheckBox,
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
}

/// Marker component for a CheckBoxWidget
#[derive(Component)]
pub struct RadioButtonWidget;

/// Marker component for a ButtonWidget
#[derive(Component)]
pub struct ButtonWidget;

// State

/// Stores the state of a toggled widget element.
#[derive(Component, Clone, Debug, Default)]
pub struct ToggleState(pub bool);
