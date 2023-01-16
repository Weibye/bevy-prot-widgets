use bevy_app::{App, Plugin};

use bevy_ecs::{
    prelude::{Bundle, Component, Entity, EntityBlueprint, IntoSystemDescriptor},
    query::Changed,
    system::{EntityCommands, Query},
};

use bevy_text::TextStyle;
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
pub struct Radio(pub bool);

#[derive(Component, Debug)]
pub struct RadioGroup {
    pub entities: Vec<Entity>,
}

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

        entity.insert((Radio(self.checked), Button, Interaction::default()));
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
    pub radio: Radio,
    // /// State of the widget
    // pub toggle: ToggleState,
    // /// The different icons for the widget
    // pub icons: RadioButtonIcons,
}

pub(crate) fn update_radio_interaction(
    mut radio: Query<(Entity, &mut Radio, &Interaction)>, // TODO: See if this can be improved with change trackers Changed<Interaction>>,
    radio_group: Query<&RadioGroup>,
) {
    let mut group_changes = vec![];

    for (entity, _, interaction) in &mut radio {
        if matches!(interaction, Interaction::Clicked) {
            // If this was clicked,
            // check if it belongs in a group
            let mut result = None;

            for group in &radio_group {
                if group.entities.iter().any(|e| *e == entity) {
                    // radio is member of this group
                    result = Some(group);
                    break;
                }
            }

            if let Some(result_group) = result {
                group_changes.push((entity, result_group));
            } else {
                // This is not part of a group, so we can just flip it here.
                // radio_widget.0 = !radio_widget.0;
            }
        }
    }

    for (clicked_entity, group) in group_changes {
        for member in &group.entities {
            if let Ok((_, mut other_radio, _)) = radio.get_mut(*member) {
                other_radio.0 = false;
            }
        }
        if let Ok((_, mut radio_widget, _)) = radio.get_mut(clicked_entity) {
            radio_widget.0 = true;
        }
    }
}

pub fn update_radio_icon(mut q: Query<(&Radio, &mut IconWidget), Changed<Radio>>) {
    for (radio, mut icon) in &mut q {
        icon.0 = if radio.0 {
            Icon::RadioButtonChecked
        } else {
            Icon::RadioButtonUnchecked
        };
    }
}
