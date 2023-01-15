//! This example showcases how to use the Radio Widget

use bevy::{
    prelude::{
        App, BuildChildren, Camera2dBundle, Color, Commands, EntityBlueprint, NodeBundle, Res,
    },
    ui::{AlignItems, FlexDirection, JustifyContent, Size, Style, Val},
    DefaultPlugins,
};
use bevy_prot_widgets::{widget::radio::RadioBlueprint, WidgetPlugin, theme::WidgetTheme};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WidgetPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_page)
        .run();
}

/// Camera
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

fn setup_page(mut cmd: Commands, theme: Res<WidgetTheme>) {
    cmd.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        background_color: Color::WHITE.into(),
        ..Default::default()
    })
    .with_children(|root| {
        for _ in 0..12 {
            RadioBlueprint {
                checked: false,
                theme: theme.icon.clone(),
            }
            .build(&mut root.spawn_empty());
        }
    });
}
