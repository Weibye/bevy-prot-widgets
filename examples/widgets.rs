//! This example illustrates the various widgets in Bevy UI.

use bevy::prelude::*;
use bevy_prot_widgets::{
    widget::{checkbox::CheckboxBundle, radio::RadioButtonBundle},
    WidgetPlugin,
};
use material_icons::Icon;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WidgetPlugin)
        // Startup
        .add_startup_system(setup_camera)
        .add_startup_system(setup)
        // .add_startup_system(load_icons)
        // .add_system(update_checkbox.after(toggle_system))
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

const BUTTON_FONT_SIZE: f32 = 15.0;
const H1_FONT_SIZE: f32 = 30.0;

const MATERIAL_FONT: &str = "fonts/MaterialIcons-Regular.ttf";
const FONT: &str = "fonts/Roboto/Roboto-Medium.ttf";

/// Camera
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let icon_style = TextStyle {
        font: asset_server.load(MATERIAL_FONT),
        font_size: 40.0,
        color: Color::DARK_GRAY,
    };

    // root node
    cmd.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        background_color: Color::WHITE.into(),
        ..default()
    })
    .with_children(|root| {
        root.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(400.), Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                margin: UiRect::all(Val::Px(5.)),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        })
        .with_children(|rect01| {
            // Buttons title
            rect01.spawn(TextBundle::from_section(
                "Buttons",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: H1_FONT_SIZE,
                    color: Color::BLACK,
                },
            ));
            // Separator
            rect01.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(1.0)),
                    margin: UiRect::new(Val::Undefined, Val::Px(5.0), Val::Undefined, Val::Px(5.0)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            });

            // Button container
            rect01
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(45.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Stretch,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|button_container| {
                    // Button 01
                    button_container
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Auto),
                                justify_content: JustifyContent::Center, // For centering button text
                                align_items: AlignItems::Center, // For centering button text
                                flex_grow: 1.,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "First button",
                                TextStyle {
                                    font: asset_server.load(FONT),
                                    font_size: BUTTON_FONT_SIZE,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // Button 02
                    button_container
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Auto),
                                // horizontally center child text
                                margin: UiRect::new(
                                    Val::Px(5.0),
                                    Val::Px(5.0),
                                    Val::Undefined,
                                    Val::Undefined,
                                ),
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                flex_grow: 1.,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Second",
                                TextStyle {
                                    font: asset_server.load(FONT),
                                    font_size: BUTTON_FONT_SIZE,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // Button 03
                    button_container
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Auto),
                                // margin: UiRect::new(Val::Px(5.0), Val::Undefined, Val::Undefined, Val::Undefined),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                flex_grow: 1.,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Third",
                                TextStyle {
                                    font: asset_server.load(FONT),
                                    font_size: BUTTON_FONT_SIZE,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });

            // Checkboxes title
            rect01.spawn(TextBundle::from_section(
                "Checkboxes",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: H1_FONT_SIZE,
                    color: Color::BLACK,
                },
            ));
            // Separator
            rect01.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(1.0)),
                    margin: UiRect::new(Val::Undefined, Val::Px(5.0), Val::Undefined, Val::Px(5.0)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            });

            // Checkbox container
            rect01
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(45.0)),
                        flex_direction: FlexDirection::Row,
                        // align_items: AlignItems::Stretch,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    // color: Color::RED.into(),
                    ..default()
                })
                .with_children(|container| {
                    container.spawn(CheckboxBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::CheckBoxOutlineBlank.to_string(),
                            icon_style.clone(),
                        ),
                        ..default()
                    });
                    container.spawn(CheckboxBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::CheckBoxOutlineBlank.to_string(),
                            icon_style.clone(),
                        ),
                        ..default()
                    });
                    container.spawn(CheckboxBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::CheckBoxOutlineBlank.to_string(),
                            icon_style.clone(),
                        ),
                        ..default()
                    });
                    container.spawn(CheckboxBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::CheckBoxOutlineBlank.to_string(),
                            icon_style.clone(),
                        ),
                        color: Color::GREEN.into(),
                        ..default()
                    });
                    container.spawn(CheckboxBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::CheckBoxOutlineBlank.to_string(),
                            icon_style.clone(),
                        ),
                        ..default()
                    });
                    container.spawn(CheckboxBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::CheckBoxOutlineBlank.to_string(),
                            icon_style.clone(),
                        ),
                        ..default()
                    });
                    container.spawn(CheckboxBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::CheckBoxOutlineBlank.to_string(),
                            icon_style.clone(),
                        ),
                        ..default()
                    });
                    container.spawn(CheckboxBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::CheckBoxOutlineBlank.to_string(),
                            icon_style.clone(),
                        ),
                        color: Color::RED.into(),
                        ..default()
                    });
                });

            // Checkboxes title
            rect01.spawn(TextBundle::from_section(
                "Radio Buttons",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: H1_FONT_SIZE,
                    color: Color::BLACK,
                },
            ));
            // Separator
            rect01.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(1.0)),
                    margin: UiRect::new(Val::Undefined, Val::Px(5.0), Val::Undefined, Val::Px(5.0)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            });

            // Checkbox container
            rect01
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(45.0)),
                        flex_direction: FlexDirection::Row,
                        // align_items: AlignItems::Stretch,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|container| {
                    container.spawn(RadioButtonBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::RadioButtonUnchecked.to_string(),
                            icon_style.clone(),
                        ),
                        ..default()
                    });
                    container.spawn(RadioButtonBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::RadioButtonUnchecked.to_string(),
                            icon_style.clone(),
                        ),
                        ..default()
                    });
                    container.spawn(RadioButtonBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::RadioButtonUnchecked.to_string(),
                            icon_style.clone(),
                        ),
                        ..default()
                    });
                    container.spawn(RadioButtonBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::RadioButtonUnchecked.to_string(),
                            icon_style.clone(),
                        ),
                        ..default()
                    });
                    container.spawn(RadioButtonBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::RadioButtonUnchecked.to_string(),
                            icon_style.clone(),
                        ),
                        ..default()
                    });
                    container.spawn(RadioButtonBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::RadioButtonUnchecked.to_string(),
                            icon_style.clone(),
                        ),
                        ..default()
                    });
                    container.spawn(RadioButtonBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::RadioButtonUnchecked.to_string(),
                            icon_style.clone(),
                        ),
                        ..default()
                    });
                    container.spawn(RadioButtonBundle {
                        // style: todo!(),
                        text: Text::from_section(
                            Icon::RadioButtonUnchecked.to_string(),
                            icon_style.clone(),
                        ),
                        ..default()
                    });
                });
        });
    });
}
