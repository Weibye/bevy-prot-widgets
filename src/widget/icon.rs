use bevy::{prelude::{Bundle, NodeBundle, Component, Commands, default, Color, Handle}, text::{Text, TextStyle, Font}, ui::CalculatedSize, ecs::system::EntityCommands};
use material_icons::Icon;

use crate::blueprint::WidgetBlueprint;

const FONT_SIZE: f32 = 20.0;
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
    fn build(self, cmd: &'w mut EntityCommands) -> &'w mut EntityCommands<'w, 's, 'a> {
        let icon_widget = IconWidgetBundle {
            icon_widget: IconWidget(self.icon.clone()),
            text: Text::from_section(
                self.icon.to_string(), 
            TextStyle { 
                font: self.font.clone(),
                font_size: FONT_SIZE, 
                color: ICON_COLOR
            }),
            ..default()
        };
        cmd.insert(icon_widget);

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

// FIXME: When spawning icon widget, user should only have to specify which icon to refer to.
// Then the _builder_ should make sure to set the correct font and text char in the text bundle