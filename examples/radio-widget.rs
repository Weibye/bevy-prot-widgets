//! This example showcases how to use the Radio Widget

use bevy::{
    prelude::{
        default, App, BuildChildren, Camera2dBundle, Color, Commands, Entity, EntityBlueprint,
        NodeBundle, Res,
    },
    ui::{AlignItems, FlexDirection, JustifyContent, Size, Style, Val},
    DefaultPlugins,
};
use bevy_prot_widgets::{
    theme::WidgetTheme,
    widget::{
        label::LabelWidgetBlueprint,
        radio::{RadioBlueprint, RadioGroup},
    },
    WidgetPlugin,
};
use bevy_ui::UiRect;

const COLOR_CONTENT_EXAMPLE: Color = Color::rgb(0.055, 0.12, 0.19);

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

// TODO: Setup a scroll-bar widget
fn setup_page(mut cmd: Commands, theme: Res<WidgetTheme>) {
    let mut radio_group: Vec<Entity> = vec![];
    // root node
    cmd.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: theme.background_color.into(),
        ..default()
    })
    .with_children(|root| {
        // Content container
        root.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Percent(100.0)),
                padding: UiRect::all(Val::Px(30.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|content| {
            LabelWidgetBlueprint {
                text: "Radio Group".into(),
                theme: theme.h1.clone(),
            }
            .build(&mut content.spawn_empty());

            LabelWidgetBlueprint {
                text: "`RadioGroup` is a helpful wrapper used to group `Radio` widgets together."
                    .into(),
                theme: theme.p.clone(),
            }
            .build(&mut content.spawn_empty());

            // Example Showcase Contaner
            content
                .spawn(NodeBundle {
                    style: Style {
                        margin: UiRect::vertical(Val::Px(10.0)),
                        padding: UiRect::vertical(Val::Px(10.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                })
                .with_children(|example_showcase| {
                    LabelWidgetBlueprint {
                        text: "Cardinal Direction".into(),
                        theme: theme.h2.clone(),
                    }
                    .build(&mut example_showcase.spawn_empty());

                    // Group
                    example_showcase
                        .spawn((NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        },))
                        .with_children(|group| {
                            for direction in ["North", "South", "East", "West"] {
                                group
                                    .spawn(NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Row,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|entry| {
                                        let mut entity = entry.spawn_empty();
                                        radio_group.push(entity.id());

                                        RadioBlueprint {
                                            checked: false,
                                            theme: theme.icon.clone(),
                                        }
                                        .build(&mut entity);

                                        LabelWidgetBlueprint {
                                            text: direction.into(),
                                            theme: theme.h3.clone(),
                                        }
                                        .build(&mut entry.spawn_empty());
                                    });
                            }
                        });
                });
        });
    });

    cmd.spawn(RadioGroup {
        entities: radio_group.into(),
    });

    //         for _ in 0..12 {
    //             RadioBlueprint {
    //                 checked: false,
    //                 theme: theme.icon.clone(),
    //             }
    //             .build(&mut root.spawn_empty());
    //         }
    //     });
}
