//! This example showcases how to use the Icon Widget

use bevy::{
    prelude::{default, App, BuildChildren, Camera2dBundle, Color, Commands, NodeBundle, Res},
    ui::{AlignItems, JustifyContent, Size, Style, Val},
    DefaultPlugins,
};
use bevy_prot_widgets::{
    blueprint::WidgetBlueprint, fonts::FontLib, widget::icon::IconWidgetBlueprint, WidgetPlugin,
};
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

fn setup_page(mut cmd: Commands, fonts: Res<FontLib>) {
    cmd.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::WHITE.into(),
        ..default()
    })
    .with_children(|root| {
        IconWidgetBlueprint {
            icon: Icon::Comment,
            font: fonts.material.clone(),
        }
        .build(&mut root.spawn_empty());

        IconWidgetBlueprint {
            icon: Icon::PlayArrow,
            font: fonts.material.clone(),
        }
        .build(&mut root.spawn_empty());

        IconWidgetBlueprint {
            icon: Icon::Pause,
            font: fonts.material.clone(),
        }
        .build(&mut root.spawn_empty());

        IconWidgetBlueprint {
            icon: Icon::Stop,
            font: fonts.material.clone(),
        }
        .build(&mut root.spawn_empty());
    });

    // TODO: Show how to change the icon during runtime
}
