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
    cmd.spawn(Camera2dBundle::default());
}

/// Creates the UI
fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let material_font = asset_server.load(MATERIAL_FONT);

    // root node
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
        for icon in [
            Icon::CheckBox,
            Icon::CheckBoxOutlineBlank,
            Icon::Check,
            Icon::CheckCircle,
        ] {
            root.spawn(TextBundle::from_section(
                icon.to_string(),
                TextStyle {
                    font: material_font.clone(),
                    font_size: 50.0,
                    color: Color::BLACK,
                },
            ));
        }
    });
}
