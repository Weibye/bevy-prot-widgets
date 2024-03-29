//! This example illustrates the various widgets in Bevy UI.

use bevy::prelude::*;
use bevy_prot_widgets::{
    theme::WidgetTheme,
    widget::{
        button::{
            IconButtonBlueprint, IconLabelButtonBlueprint, LabelButtonBlueprint, TriggerPolicy,
        },
        checkbox::{CheckBoxBlueprint, CheckboxState},
        icon::IconWidgetBlueprint,
        label::LabelWidgetBlueprint,
        radio::RadioBlueprint,
    },
    WidgetPlugin,
};
use material_icons::Icon;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WidgetPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/// Camera
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

fn setup(mut cmd: Commands, theme: Res<WidgetTheme>) {
    // root node
    cmd.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: theme.background_color.into(),
        ..default()
    })
    .with_children(|root| {
        // Buttons title
        LabelWidgetBlueprint {
            text: "Buttons".into(),
            theme: theme.h1.clone(),
        }
        .build(&mut root.spawn_empty());

        // TODO: Horizontal line widget
        // Separator
        root.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(1.0)),
                margin: UiRect::new(Val::Undefined, Val::Px(5.0), Val::Undefined, Val::Px(5.0)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        });

        // Button container
        root.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Undefined, Val::Px(45.0)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|button_container| {
            // Button 01
            LabelButtonBlueprint {
                label: LabelWidgetBlueprint {
                    text: "Click me!".into(),
                    theme: theme.button.clone(),
                },
                enabled: true,
                policy: TriggerPolicy::OnRelease,
            }
            .build(&mut button_container.spawn_empty());

            // Button 02
            IconButtonBlueprint {
                icon: IconWidgetBlueprint {
                    icon: Icon::Save,
                    theme: theme.icon.clone(),
                },
                enabled: true,
                policy: TriggerPolicy::OnRelease,
            }
            .build(&mut button_container.spawn_empty());

            // Button 03
            IconLabelButtonBlueprint {
                label: LabelWidgetBlueprint {
                    text: "Send".into(),
                    theme: theme.button.clone(),
                },
                icon: IconWidgetBlueprint {
                    icon: Icon::Send,
                    theme: theme.icon.clone(),
                },
                enabled: true,
                policy: TriggerPolicy::OnRelease,
            }
            .build(&mut button_container.spawn_empty());
        });

        // Checkboxes title
        LabelWidgetBlueprint {
            text: "Check Boxes".into(),
            theme: theme.h1.clone(),
        }
        .build(&mut root.spawn_empty());

        // Separator
        root.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(1.0)),
                margin: UiRect::new(Val::Undefined, Val::Px(5.0), Val::Undefined, Val::Px(5.0)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        });

        // Checkbox container
        root.spawn(NodeBundle {
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
            for _ in 0..10 {
                CheckBoxBlueprint {
                    state: CheckboxState::Unchecked,
                    theme: theme.icon.clone(),
                }
                .build(&mut container.spawn_empty());
            }
        });

        // Radio buttons title
        LabelWidgetBlueprint {
            text: "Radio Buttons".into(),
            theme: theme.h1.clone(),
        }
        .build(&mut root.spawn_empty());

        // Separator
        root.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(1.0)),
                margin: UiRect::new(Val::Undefined, Val::Px(5.0), Val::Undefined, Val::Px(5.0)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        });

        // Checkbox container
        root.spawn(NodeBundle {
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
            for _ in 0..10 {
                RadioBlueprint {
                    checked: false,
                    theme: theme.icon.clone(),
                }
                .build(&mut container.spawn_empty());
            }
        });
    });
}
