use bevy::{
    ecs::system::EntityCommands,
    prelude::{default, Bundle, Color, Component, Handle, NodeBundle},
    text::{Font, Text, TextStyle},
    ui::CalculatedSize,
};
use material_icons::Icon;

use crate::blueprint::WidgetBlueprint;

const FONT_SIZE: f32 = 50.0;
const ICON_COLOR: Color = Color::BLACK;

#[derive(Component)]
pub struct IconWidget(pub Icon);

impl Default for IconWidget {
    fn default() -> Self {
        Self(Icon::CheckCircle)
    }
}

pub struct IconWidgetBlueprint {
    pub icon: Icon,
    pub font: Handle<Font>,
}

impl<'w, 's> WidgetBlueprint<'w, 's> for IconWidgetBlueprint {
    fn build<'a>(
        self,
        cmd: &'a mut EntityCommands<'w, 's, 'a>,
    ) -> &'a mut EntityCommands<'w, 's, 'a> {
        cmd.insert(IconWidgetBundle {
            icon_widget: IconWidget(self.icon),
            text: Text::from_section(
                self.icon.to_string(),
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
pub struct IconWidgetBundle {
    #[bundle]
    pub node_bundle: NodeBundle,
    pub icon_widget: IconWidget,
    pub text: Text,
    /// The calculated size based on the given image
    pub calculated_size: CalculatedSize,
}
