use bevy_asset::Handle;
use bevy_ecs::{
    prelude::{Bundle, Component, EntityBlueprint},
    query::Changed,
    system::{EntityCommands, Query},
};
use bevy_render::prelude::Color;
use bevy_text::{Font, Text, TextStyle};
use bevy_ui::{prelude::NodeBundle, CalculatedSize};

const FONT_SIZE: f32 = 50.0;
const ICON_COLOR: Color = Color::BLACK;

#[derive(Component, Default)]
pub struct LabelWidget {
    pub text: String,
}

pub struct LabelWidgetBlueprint {
    pub text: String,
    pub font: Handle<Font>,
}

impl<'w, 's> EntityBlueprint for LabelWidgetBlueprint {
    fn build<'a>(self, cmd: &'a mut EntityCommands) {
        cmd.insert(LabelWidgetBundle {
            label: LabelWidget {
                text: self.text.clone(),
            },
            text: Text::from_section(
                self.text,
                TextStyle {
                    font: self.font,
                    font_size: FONT_SIZE,
                    color: ICON_COLOR,
                },
            ),
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
