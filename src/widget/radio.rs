use bevy::prelude::{Changed, Query};
use bevy::render::color::Color;
use bevy::{
    ecs::system::EntityCommands,
    prelude::{default, Bundle, Button, Component, Handle},
    text::{Font, Text, TextStyle},
    ui::Interaction,
};
use material_icons::Icon;

use crate::blueprint::WidgetBlueprint;

use super::icon::{IconWidget, IconWidgetBundle};

/// Marker component for a CheckBoxWidget
#[derive(Component, Debug, Clone, Default)]
pub struct RadioWidget(pub bool);

const FONT_SIZE: f32 = 50.0;
const ICON_COLOR: Color = Color::BLACK;

pub struct RadioBlueprint {
    pub checked: bool,
    pub font: Handle<Font>,
}

impl<'w, 's> WidgetBlueprint<'w, 's> for RadioBlueprint {
    fn build<'a>(
        self,
        cmd: &'a mut EntityCommands<'w, 's, 'a>,
    ) -> &'a mut EntityCommands<'w, 's, 'a> {
        let icon = if self.checked {
            Icon::RadioButtonChecked
        } else {
            Icon::RadioButtonUnchecked
        };

        cmd.insert(RadioBundle {
            radio: RadioWidget(self.checked),
            icon: IconWidgetBundle {
                icon_widget: IconWidget(icon),
                text: Text::from_section(
                    icon.to_string(),
                    TextStyle {
                        font: self.font,
                        font_size: FONT_SIZE,
                        color: ICON_COLOR,
                    },
                ),
                ..default()
            },
            ..default()
        });

        cmd
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

pub(crate) fn update_radio_icon(
    mut q: Query<(&RadioWidget, &mut IconWidget), Changed<RadioWidget>>,
) {
    for (radio, mut icon) in &mut q {
        icon.0 = if radio.0 {
            Icon::RadioButtonChecked
        } else {
            Icon::RadioButtonUnchecked
        };
    }
}
