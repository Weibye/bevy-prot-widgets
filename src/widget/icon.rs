use bevy_app::{App, Plugin};
use bevy_ecs::{
    prelude::{Bundle, Component, EntityBlueprint, IntoSystemDescriptor},
    query::Changed,
    system::{EntityCommands, Query},
};
use bevy_text::{Text, TextStyle};
use bevy_ui::{prelude::NodeBundle, CalculatedSize, Style, UiRect, Val};
use material_icons::Icon;

use super::label::update_changed_labels;

pub(crate) struct IconPlugin;

impl Plugin for IconPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_changed_icons.before(update_changed_labels));
    }
}

#[derive(Component, Debug)]
pub struct IconWidget(pub Icon);

impl Default for IconWidget {
    fn default() -> Self {
        Self(Icon::CheckCircle)
    }
}

pub struct IconWidgetBlueprint {
    pub icon: Icon,
    pub theme: TextStyle,
}

/// Defines how to construct the IconWidget.
impl<'w, 's> EntityBlueprint for IconWidgetBlueprint {
    fn build<'a>(self, entity: &'a mut EntityCommands) {
        entity.insert(IconWidgetBundle {
            icon_widget: IconWidget(self.icon),
            text: Text::from_section(self.icon.to_string(), self.theme),
            node_bundle: NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

#[derive(Bundle, Default, Debug)]
pub struct IconWidgetBundle {
    #[bundle]
    pub node_bundle: NodeBundle,
    pub icon_widget: IconWidget,
    pub text: Text,
    /// The calculated size based on the given image
    pub calculated_size: CalculatedSize,
}

pub(crate) fn update_changed_icons(mut q: Query<(&IconWidget, &mut Text), Changed<IconWidget>>) {
    for (widget, mut text) in &mut q {
        text.sections[0].value = widget.0.to_string();
    }
}
