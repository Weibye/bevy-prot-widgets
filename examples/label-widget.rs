//! This example showcases how to use the Label Widget

use bevy::{
    prelude::{
        App, BuildChildren, Camera2dBundle, Color, Commands, EntityBlueprint, NodeBundle, Res,
    },
    ui::{AlignItems, FlexDirection, JustifyContent, Size, Style, Val},
    DefaultPlugins,
};
use bevy_prot_widgets::{theme::WidgetTheme, widget::label::LabelWidgetBlueprint, WidgetPlugin};

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
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        background_color: Color::WHITE.into(),
        ..Default::default()
    })
    .with_children(|root| {
        for (label, text_style) in [
            ("This is a H1 header", theme.h1.clone()),
            ("This is a H2 header", theme.h2.clone()),
            ("This is a H3 header", theme.h3.clone()),
            (
                "This is a paragraph, and it is a bit longer.",
                theme.p.clone(),
            ),
        ] {
            LabelWidgetBlueprint {
                text: label.into(),
                theme: text_style,
            }
            .build(&mut root.spawn_empty());
        }
    });
}
