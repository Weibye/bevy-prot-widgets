use bevy::prelude::*;

use bevy_prot_widgets::{
    theme::WidgetTheme,
    widget::{
        button::{
            ButtonColor, ButtonTheme, IconButtonBlueprint, IconLabelButtonBlueprint,
            LabelButtonBlueprint, TriggerPolicy,
        },
        icon::IconWidgetBlueprint,
        label::LabelWidgetBlueprint,
    },
    WidgetPlugin,
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

// TODO: Show buttons with labels in different positions
// TODO: Show buttons with different themes / styles
// TODO: Show that clicking buttons actually change something

const COLOR_CONTENT_EXAMPLE: Color = Color::rgb(0.055, 0.12, 0.19);

// TODO: Button should not change on hover
const BUTTON_THEME: ButtonTheme = ButtonTheme {
    background: ButtonColor {
        pressed: Color::rgb(0.7, 0.7, 0.7),
        released: Color::rgb(0.6, 0.6, 0.6),
        hovered: Color::rgb(0.5, 0.5, 0.5),
        default: Color::rgb(0.25, 0.25, 0.25),
        disabled: Color::rgb(0.1, 0.1, 0.1),
    },
    foreground: ButtonColor {
        pressed: Color::rgb(0.9, 0.5, 0.1),
        released: Color::rgb(0.905, 0.921, 0.941),
        hovered: Color::rgb(0.905, 0.921, 0.941),
        default: Color::rgb(0.905, 0.921, 0.941),
        disabled: Color::rgb(0.35, 0.35, 0.35),
    },
};

/// Camera
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

// TODO: Setup a scroll-bar widget
fn setup_page(mut cmd: Commands, theme: Res<WidgetTheme>) {
    // root node
    cmd.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: theme.background_color.into(),
            ..default()
        }).with_children(| root| {
            // Content container
            root.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Percent(100.0)),
                    padding: UiRect::all(Val::Px(30.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            }).with_children(| content | {

                LabelWidgetBlueprint {
                    text: "Buttons".into(),
                    theme: theme.h1.clone(),
                }.build(&mut content.spawn_empty());

                LabelWidgetBlueprint {
                    text: "Buttons are used to trigger actions. \
                    They can be hovered and clicked. \
                    The most basic button is the text button".into(),
                    theme: theme.p.clone(),
                }.build(&mut content.spawn_empty());

                // Example Showcase Contaner
                content.spawn(NodeBundle {
                    style: Style {
                        margin: UiRect::vertical(Val::Px(10.0)),
                        padding: UiRect::vertical(Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {

                    LabelButtonBlueprint {
                        label: LabelWidgetBlueprint {
                            text: "Open inventory".into(),
                            theme: theme.button.clone(),
                        },
                        enabled: true,
                        policy: TriggerPolicy::OnRelease,
                    }.build(&mut example_showcase.spawn_empty());

                    IconButtonBlueprint {
                        icon: IconWidgetBlueprint {
                            icon: Icon::Menu,
                            theme: theme.icon.clone(),
                        },
                        enabled: true,
                        policy: TriggerPolicy::OnRelease,
                    }.build(&mut example_showcase.spawn_empty());

                    IconLabelButtonBlueprint {
                        icon: IconWidgetBlueprint { icon: Icon::Send, theme: theme.icon.clone() },
                        label: LabelWidgetBlueprint { text: "Send".into(), theme: theme.button.clone() },
                        enabled: true,
                        policy: TriggerPolicy::OnRelease,
                    }.build(&mut example_showcase.spawn_empty());
                });

                LabelWidgetBlueprint {
                    text: "Disabled Buttons".into(),
                    theme: theme.h1.clone(),
                }.build(&mut content.spawn_empty());

                LabelWidgetBlueprint {
                    text: "Buttons can be either enabled or disabled. \
                    Disabled buttons should not be triggered by the user. \
                    Buttons should clearly show when they are disabled by changing colors.".into(),
                    theme: theme.p.clone(),
                }.build(&mut content.spawn_empty());
                // Example Showcase
                content.spawn(NodeBundle {
                    style: Style {
                        margin: UiRect::vertical(Val::Px(10.0)),
                        padding: UiRect::vertical(Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {

                    // Spawn text-buttons
                    for text in ["Ok", "Submit", "Restart Game"] {
                        LabelButtonBlueprint {
                            label: LabelWidgetBlueprint {
                                text: text.into(),
                                theme: theme.button.clone(),
                            },
                            enabled: false,
                            policy: TriggerPolicy::OnRelease,
                        }.build(&mut example_showcase.spawn_empty());
                    }
                });

                LabelWidgetBlueprint {
                    text: "Trigger Policy".into(),
                    theme: theme.h1.clone(),
                }.build(&mut content.spawn_empty());

                LabelWidgetBlueprint {
                    text: "Buttons are triggered on release by default. \
                    By setting the trigger-policy you can change the button to trigger on press instead.".into(),
                    theme: theme.p.clone(),
                }.build(&mut content.spawn_empty());

                // Example Showcase
                content.spawn(NodeBundle {
                    style: Style {
                        margin: UiRect::vertical(Val::Px(10.0)),
                        padding: UiRect::vertical(Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {

                    LabelButtonBlueprint {
                        label: LabelWidgetBlueprint {
                            text: "Ok".into(),
                            theme: theme.button.clone(),
                        },
                        enabled: true,
                        policy: TriggerPolicy::OnPress,
                    }.build(&mut example_showcase.spawn_empty());

                    IconLabelButtonBlueprint {
                        icon: IconWidgetBlueprint { icon: Icon::Wifi, theme: theme.icon.clone() },
                        label: LabelWidgetBlueprint { text: "Enable Wifi".into(), theme: theme.button.clone() },
                        enabled: true,
                        policy: TriggerPolicy::OnRelease,
                    }.build(&mut example_showcase.spawn_empty());
                });

                LabelWidgetBlueprint {
                    text: "Icon Buttons".into(),
                    theme: theme.h1.clone(),
                }.build(&mut content.spawn_empty());

                LabelWidgetBlueprint {
                    text: "Some buttons only need icon.".into(),
                    theme: theme.p.clone(),
                }.build(&mut content.spawn_empty());

                // Example Showcase
                content.spawn(NodeBundle {
                    style: Style {
                        margin: UiRect::vertical(Val::Px(10.0)),
                        padding: UiRect::vertical(Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {

                    // Spawn icon-buttons
                    for icon in [Icon::Wifi, Icon::Subtitles, Icon::Delete, Icon::Add, Icon::Home] {
                        IconButtonBlueprint {
                            icon: IconWidgetBlueprint {
                                icon,
                                theme: theme.icon.clone(),
                            },
                            enabled: true,
                            policy: TriggerPolicy::OnRelease,
                        }.build(&mut example_showcase.spawn_empty());
                    }
                });

                LabelWidgetBlueprint {
                    text: "Icon and Text Buttons".into(),
                    theme: theme.h1.clone(),
                }.build(&mut content.spawn_empty());

                LabelWidgetBlueprint {
                    text: "We can also create buttons that has both icons and text.".into(),
                    theme: theme.p.clone(),
                }.build(&mut content.spawn_empty());

                // Example Showcase
                content.spawn(NodeBundle {
                    style: Style {
                        margin: UiRect::vertical(Val::Px(10.0)),
                        padding: UiRect::vertical(Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {

                    // Spawn icon-buttons
                    for (icon, text) in [
                        (Icon::Wifi, "Enable Wifi"), 
                        (Icon::Subtitles, "Toggle Subtitles"), 
                        (Icon::Delete, "Delete Item"), 
                        (Icon::Add, "Add item"), 
                        (Icon::Home, "Home")
                    ] {
                        IconLabelButtonBlueprint {
                            icon: IconWidgetBlueprint { icon, theme: theme.icon.clone() },
                            label: LabelWidgetBlueprint { text: text.into(), theme: theme.button.clone() },
                            enabled: true,
                            policy: TriggerPolicy::OnRelease,
                        }.build(&mut example_showcase.spawn_empty());
                    }
                });
            });
        });
}
