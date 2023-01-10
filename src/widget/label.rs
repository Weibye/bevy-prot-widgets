use bevy::{
    ecs::system::EntityCommands,
    prelude::{default, Bundle, Color, Component, Handle, NodeBundle},
    text::{Font, Text, TextStyle},
    ui::CalculatedSize,
};

use crate::blueprint::WidgetBlueprint;

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

impl<'w, 's> WidgetBlueprint<'w, 's> for LabelWidgetBlueprint {
    fn build<'a>(
        self,
        cmd: &'a mut EntityCommands<'w, 's, 'a>,
    ) -> &'a mut EntityCommands<'w, 's, 'a> {
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
            ..default()
        });

        cmd
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
