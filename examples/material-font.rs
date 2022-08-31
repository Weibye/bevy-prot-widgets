//! Example that showcases how to use the material-icon fonts

use bevy::prelude::*;
use material_icons::Icon;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(setup)
        .run();
}

const MATERIAL_FONT: &str = "fonts/MaterialIcons-Regular.ttf";

/// Creates the camera needed for UI rendering
fn setup_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());
}

/// Creates the UI
fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // root node
    cmd.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: Color::WHITE.into(),
        ..default()
    })
    .with_children(|root| {
        root.spawn_bundle(TextBundle::from_section(
            Icon::CheckBox.to_string(),
            TextStyle {
                font: asset_server.load(MATERIAL_FONT),
                font_size: 50.0,
                color: Color::BLACK,
            },
        ));

        root.spawn_bundle(TextBundle::from_section(
            Icon::CheckBoxOutlineBlank.to_string(),
            TextStyle {
                font: asset_server.load(MATERIAL_FONT),
                font_size: 50.0,
                color: Color::BLACK,
            },
        ));

        root.spawn_bundle(TextBundle::from_section(
            Icon::Check.to_string(),
            TextStyle {
                font: asset_server.load(MATERIAL_FONT),
                font_size: 50.0,
                color: Color::BLACK,
            },
        ));

        root.spawn_bundle(TextBundle::from_section(
            Icon::CheckCircle.to_string(),
            TextStyle {
                font: asset_server.load(MATERIAL_FONT),
                font_size: 50.0,
                color: Color::BLACK,
            },
        ));
    });
}
