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

pub(crate) struct CheckBoxPlugin;

impl Plugin for CheckBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_checkbox_icon.before(update_changed_icons))
            .add_system(update_checkbox_interaction.before(update_checkbox_icon));
    }
}

/// Marker component for a [CheckboxWidget].
#[derive(Component, Debug, Clone, Default)]
pub struct CheckboxWidget(pub CheckboxState);

#[derive(Debug, Clone, Default)]
pub enum CheckboxState {
    Checked,
    #[default]
    Unchecked,
    Indeterminate,
}

pub struct CheckBoxBlueprint {
    pub state: CheckboxState,
    pub theme: TextStyle,
}

impl<'w, 's> EntityBlueprint for CheckBoxBlueprint {
    fn build<'a>(self, entity: &'a mut EntityCommands) {
        let icon = match self.state {
            CheckboxState::Checked => Icon::CheckBox,
            CheckboxState::Unchecked => Icon::CheckBoxOutlineBlank,
            CheckboxState::Indeterminate => Icon::IndeterminateCheckBox,
        };

        IconWidgetBlueprint {
            icon,
            theme: self.theme,
        }
        .build(entity);

        entity.insert((CheckboxWidget(self.state), Button, Interaction::default()));
    }
}

/// A Checkbox Widget
#[derive(Bundle, Debug, Default)]
pub struct CheckboxBundle {
    pub checkbox: CheckboxWidget,
    #[bundle]
    pub icon: IconWidgetBundle,

    pub interaction: Interaction,

    pub button: Button,
}

pub(crate) fn update_checkbox_interaction(
    mut q: Query<(&mut CheckboxWidget, &Interaction), Changed<Interaction>>,
) {
    for (mut checkbox, interaction) in &mut q {
        if matches!(interaction, Interaction::Clicked) {
            checkbox.0 = match checkbox.0 {
                CheckboxState::Checked => CheckboxState::Unchecked,
                CheckboxState::Unchecked => CheckboxState::Checked,
                CheckboxState::Indeterminate => CheckboxState::Checked,
            }
        }
    }
}

pub(crate) fn update_checkbox_icon(
    mut q: Query<(&CheckboxWidget, &mut IconWidget), Changed<CheckboxWidget>>,
) {
    for (checkbox, mut icon) in &mut q {
        icon.0 = match checkbox.0 {
            CheckboxState::Checked => Icon::CheckBox,
            CheckboxState::Unchecked => Icon::CheckBoxOutlineBlank,
            CheckboxState::Indeterminate => Icon::IndeterminateCheckBox,
        };
    }
}
