use bevy_app::{App, Plugin};

use bevy_ecs::{
    prelude::{Bundle, Component, EntityBlueprint, IntoSystemDescriptor},
    query::Changed,
    system::{EntityCommands, Query},
};

use bevy_text::{Text, TextStyle};
use bevy_ui::{widget::Button, Interaction};
use material_icons::Icon;

use super::icon::{update_changed_icons, IconWidget, IconWidgetBlueprint, IconWidgetBundle};

pub(crate) struct RadioPlugin;

impl Plugin for RadioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_radio_icon.before(update_changed_icons))
            .add_system(update_radio_interaction.before(update_radio_icon));
    }
}

/// Marker component for a CheckBoxWidget
#[derive(Component, Debug, Clone, Default)]
pub struct RadioWidget(pub bool);

pub struct RadioBlueprint {
    pub checked: bool,
    pub theme: TextStyle,
}

impl EntityBlueprint for RadioBlueprint {
    fn build(self, entity: &mut EntityCommands) {
        let icon = if self.checked {
            Icon::RadioButtonChecked
        } else {
            Icon::RadioButtonUnchecked
        };

        IconWidgetBlueprint {
            icon,
            theme: self.theme,
        }
        .build(entity);

        entity.insert((RadioWidget(self.checked), Button, Interaction::default()));
    }
}

/// A Checkbox Widget
#[derive(Bundle, Debug, Default)]
pub struct RadioBundle {
    #[bundle]
    pub icon: IconWidgetBundle,
    // Additional stuff
    /// Enables clicking on the widget
    pub button: Button,
    /// Interaction state of the widget
    pub interaction: Interaction,
    /// Marker to make it a "CheckboxWidget"
    pub radio: RadioWidget,
    // /// State of the widget
    // pub toggle: ToggleState,
    // /// The different icons for the widget
    // pub icons: RadioButtonIcons,
}

pub(crate) fn update_radio_interaction(
    mut q: Query<(&mut RadioWidget, &Interaction), Changed<Interaction>>,
) {
    for (mut radio, interaction) in &mut q {
        if matches!(interaction, Interaction::Clicked) {
            radio.0 = !radio.0;
        }
    }
}

pub fn update_radio_icon(mut q: Query<(&RadioWidget, &mut IconWidget), Changed<RadioWidget>>) {
    for (radio, mut icon) in &mut q {
        icon.0 = if radio.0 {
            Icon::RadioButtonChecked
        } else {
            Icon::RadioButtonUnchecked
        };
    }
}
