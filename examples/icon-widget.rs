//! This example showcases how to use the Icon Widget

use bevy::{
    prelude::{
        App, BuildChildren, Camera2dBundle, Color, Commands, EntityBlueprint, NodeBundle, Res,
    },
    ui::{AlignItems, JustifyContent, Size, Style, Val},
    DefaultPlugins,
};
use bevy_prot_widgets::{theme::WidgetTheme, widget::icon::IconWidgetBlueprint, WidgetPlugin};
use material_icons::Icon;

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
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: Color::WHITE.into(),
        ..Default::default()
    })
    .with_children(|root| {
        for icon in [Icon::Comment, Icon::PlayArrow, Icon::Pause, Icon::Stop] {
            IconWidgetBlueprint {
                icon,
                theme: theme.icon.clone(),
            }
            .build(&mut root.spawn_empty());
        }
    });
}
