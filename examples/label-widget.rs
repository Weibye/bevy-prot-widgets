//! This example showcases how to use the Label Widget

use bevy::{
    prelude::{App, BuildChildren, Camera2dBundle, Color, Commands, NodeBundle, Res, EntityBlueprint},
    ui::{AlignItems, FlexDirection, JustifyContent, Size, Style, Val},
    DefaultPlugins,
};
use bevy_prot_widgets::{ fonts::FontLib, widget::label::LabelWidgetBlueprint, WidgetPlugin,
};

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

fn setup_page(mut cmd: Commands, fonts: Res<FontLib>) {
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

        for label in ["This is a text-label widget", "This is a different one", "Some label", "Fourth label"] {
            LabelWidgetBlueprint {
                text: label.into(),
                font: fonts.normal.clone(),
            }
            .build(&mut root.spawn_empty());
        }
    });
}
