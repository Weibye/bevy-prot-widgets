//! This example illustrates the various widgets in Bevy UI.

use bevy::prelude::*;
use bevy_widgets::*;
use material_icons::{icon_to_char, Icon};

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

fn setup_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon_style = TextStyle {
        font: asset_server.load(MATERIAL_FONT),
        font_size: 40.0,
        color: Color::DARK_GRAY,
    };

    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            // color: Color::rgb(0.9, 0.9, 0.9).into(),
            color: Color::WHITE.into(),
            ..default()
        })
        .with_children(|root| {
            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(400.), Val::Auto),
                    flex_direction: FlexDirection::ColumnReverse,
                    margin: UiRect::all(Val::Px(5.)),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                // color: Color::rgb(0.5, 0.5, 0.5).into(),
                color: Color::WHITE.into(),
                ..default()
            })
            .with_children(|rect01| {
                // // Buttons title
                // rect01.spawn_bundle(TextBundle::from_section(
                //     "Buttons",
                //     TextStyle {
                //         font: asset_server.load(FONT),
                //         font_size: H1_FONT_SIZE,
                //         color: Color::BLACK,
                //     },
                // ));
                // // Separator
                // rect01.spawn_bundle(NodeBundle {
                //     style: Style {
                //         size: Size::new(Val::Percent(100.0), Val::Px(1.0)),
                //         margin: UiRect::new(
                //             Val::Undefined,
                //             Val::Px(5.0),
                //             Val::Undefined,
                //             Val::Px(5.0),
                //         ),
                //         ..default()
                //     },
                //     color: Color::BLACK.into(),
                //     ..default()
                // });

                // // Button container
                // rect01
                //     .spawn_bundle(NodeBundle {
                //         style: Style {
                //             size: Size::new(Val::Auto, Val::Px(45.0)),
                //             flex_direction: FlexDirection::Row,
                //             align_items: AlignItems::Stretch,
                //             justify_content: JustifyContent::SpaceBetween,
                //             ..default()
                //         },
                //         ..default()
                //     })
                //     .with_children(|button_container| {
                //         // Button 01
                //         button_container
                //             .spawn_bundle(ButtonBundle {
                //                 style: Style {
                //                     size: Size::new(Val::Auto, Val::Auto),
                //                     justify_content: JustifyContent::Center, // For centering button text
                //                     align_items: AlignItems::Center, // For centering button text
                //                     flex_grow: 1.,
                //                     ..default()
                //                 },
                //                 color: NORMAL_BUTTON.into(),
                //                 ..default()
                //             })
                //             .with_children(|parent| {
                //                 parent.spawn_bundle(TextBundle::from_section(
                //                     "First button",
                //                     TextStyle {
                //                         font: asset_server.load(FONT),
                //                         font_size: BUTTON_FONT_SIZE,
                //                         color: Color::WHITE,
                //                     },
                //                 ));
                //             });

                //         // Button 02
                //         button_container
                //             .spawn_bundle(ButtonBundle {
                //                 style: Style {
                //                     size: Size::new(Val::Auto, Val::Auto),
                //                     // horizontally center child text
                //                     margin: UiRect::new(
                //                         Val::Px(5.0),
                //                         Val::Px(5.0),
                //                         Val::Undefined,
                //                         Val::Undefined,
                //                     ),
                //                     justify_content: JustifyContent::Center,
                //                     // vertically center child text
                //                     align_items: AlignItems::Center,
                //                     flex_grow: 1.,
                //                     ..default()
                //                 },
                //                 color: NORMAL_BUTTON.into(),
                //                 ..default()
                //             })
                //             .with_children(|parent| {
                //                 parent.spawn_bundle(TextBundle::from_section(
                //                     "Second",
                //                     TextStyle {
                //                         font: asset_server.load(FONT),
                //                         font_size: BUTTON_FONT_SIZE,
                //                         color: Color::WHITE,
                //                     },
                //                 ));
                //             });

                //         // Button 03
                //         button_container
                //             .spawn_bundle(ButtonBundle {
                //                 style: Style {
                //                     size: Size::new(Val::Auto, Val::Auto),
                //                     // margin: UiRect::new(Val::Px(5.0), Val::Undefined, Val::Undefined, Val::Undefined),
                //                     // horizontally center child text
                //                     justify_content: JustifyContent::Center,
                //                     // vertically center child text
                //                     align_items: AlignItems::Center,
                //                     flex_grow: 1.,
                //                     ..default()
                //                 },
                //                 color: NORMAL_BUTTON.into(),
                //                 ..default()
                //             })
                //             .with_children(|parent| {
                //                 parent.spawn_bundle(TextBundle::from_section(
                //                     "Third",
                //                     TextStyle {
                //                         font: asset_server.load(FONT),
                //                         font_size: BUTTON_FONT_SIZE,
                //                         color: Color::WHITE,
                //                     },
                //                 ));
                //             });
                //     });

                // Checkboxes title
                rect01.spawn_bundle(TextBundle::from_section(
                    "Checkboxes",
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: H1_FONT_SIZE,
                        color: Color::BLACK,
                    },
                ));
                // Separator
                rect01.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(1.0)),
                        margin: UiRect::new(
                            Val::Undefined,
                            Val::Px(5.0),
                            Val::Undefined,
                            Val::Px(5.0),
                        ),
                        ..default()
                    },
                    color: Color::BLACK.into(),
                    ..default()
                });

                // Checkbox container
                rect01
                    .spawn_bundle(NodeBundle {
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
                        container.spawn_bundle(CheckboxBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                icon_to_char(Icon::CheckBoxOutlineBlank),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                        container.spawn_bundle(CheckboxBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                icon_to_char(Icon::CheckBoxOutlineBlank),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                        container.spawn_bundle(CheckboxBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                icon_to_char(Icon::CheckBoxOutlineBlank),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                        container.spawn_bundle(CheckboxBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                icon_to_char(Icon::CheckBoxOutlineBlank),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                        container.spawn_bundle(CheckboxBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                icon_to_char(Icon::CheckBoxOutlineBlank),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                        container.spawn_bundle(CheckboxBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                icon_to_char(Icon::CheckBoxOutlineBlank),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                        container.spawn_bundle(CheckboxBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                icon_to_char(Icon::CheckBoxOutlineBlank),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                        container.spawn_bundle(CheckboxBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                icon_to_char(Icon::CheckBoxOutlineBlank),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                    });

                // Checkboxes title
                rect01.spawn_bundle(TextBundle::from_section(
                    "Radio Buttons",
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: H1_FONT_SIZE,
                        color: Color::BLACK,
                    },
                ));
                // Separator
                rect01.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(1.0)),
                        margin: UiRect::new(
                            Val::Undefined,
                            Val::Px(5.0),
                            Val::Undefined,
                            Val::Px(5.0),
                        ),
                        ..default()
                    },
                    color: Color::BLACK.into(),
                    ..default()
                });

                // Checkbox container
                rect01
                    .spawn_bundle(NodeBundle {
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
                        container.spawn_bundle(RadioButtonBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                Icon::RadioButtonUnchecked.to_string(),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                        container.spawn_bundle(RadioButtonBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                Icon::RadioButtonUnchecked.to_string(),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                        container.spawn_bundle(RadioButtonBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                Icon::RadioButtonUnchecked.to_string(),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                        container.spawn_bundle(RadioButtonBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                Icon::RadioButtonUnchecked.to_string(),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                        container.spawn_bundle(RadioButtonBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                Icon::RadioButtonUnchecked.to_string(),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                        container.spawn_bundle(RadioButtonBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                Icon::RadioButtonUnchecked.to_string(),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                        container.spawn_bundle(RadioButtonBundle {
                            // style: todo!(),
                            text: Text::from_section(
                                Icon::RadioButtonUnchecked.to_string(),
                                icon_style.clone(),
                            ),
                            ..default()
                        });
                        container.spawn_bundle(RadioButtonBundle {
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
