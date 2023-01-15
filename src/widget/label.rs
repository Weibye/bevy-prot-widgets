use bevy_app::{App, Plugin};
use bevy_ecs::{
    prelude::{Bundle, Component, EntityBlueprint},
    query::Changed,
    system::{EntityCommands, Query},
};
use bevy_text::{Text, TextStyle};
use bevy_ui::{prelude::NodeBundle, CalculatedSize};

pub(crate) struct LabelPlugin;

impl Plugin for LabelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_changed_labels);
    }
}

#[derive(Component, Default)]
pub struct LabelWidget {
    pub text: String,
}

pub struct LabelWidgetBlueprint {
    pub text: String,
    pub theme: TextStyle,
}

impl<'w, 's> EntityBlueprint for LabelWidgetBlueprint {
    fn build<'a>(self, cmd: &'a mut EntityCommands) {
        cmd.insert(LabelWidgetBundle {
            label: LabelWidget {
                text: self.text.clone(),
            },
            text: Text::from_section(self.text, self.theme),
            ..Default::default()
        });
    }
}

#[derive(Bundle, Default)]
pub struct LabelWidgetBundle {
    #[bundle]
    pub node_bundle: NodeBundle,
    pub label: LabelWidget,
    pub text: Text,
    pub calculated_size: CalculatedSize,
}

pub(crate) fn update_changed_labels(mut q: Query<(&LabelWidget, &mut Text), Changed<LabelWidget>>) {
    for (widget, mut text) in &mut q {
        text.sections[0].value = widget.text.clone();
    }
}
