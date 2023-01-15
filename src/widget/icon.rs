use bevy_asset::Handle;
use bevy_ecs::{
    prelude::{Bundle, Component, EntityBlueprint},
    query::Changed,
    system::{EntityCommands, Query},
};
use bevy_render::prelude::Color;
use bevy_text::{Font, Text, TextStyle};
use bevy_ui::{prelude::NodeBundle, CalculatedSize};
use material_icons::Icon;

const FONT_SIZE: f32 = 50.0;
const ICON_COLOR: Color = Color::BLACK;

#[derive(Component, Debug)]
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

impl<'w, 's> EntityBlueprint for IconWidgetBlueprint {
    fn build<'a>(self, cmd: &'a mut EntityCommands) {
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
