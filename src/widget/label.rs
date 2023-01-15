use bevy_app::{App, Plugin};
use bevy_ecs::{
    prelude::{Bundle, Component, EntityBlueprint},
    query::Changed,
    system::{EntityCommands, Query},
};
use bevy_text::{Text, TextStyle};
use bevy_ui::{prelude::NodeBundle, CalculatedSize, Style, UiRect, Val};

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

impl EntityBlueprint for LabelWidgetBlueprint {
    fn build(self, cmd: &mut EntityCommands) {
        cmd.insert(LabelWidgetBundle {
            label: LabelWidget {
                text: self.text.clone(),
            },
            text: Text::from_section(self.text, self.theme),
            node_bundle: NodeBundle {
                style: Style {
                    margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(8.0), Val::Px(5.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
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
