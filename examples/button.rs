use bevy::{prelude::*, ecs::system::Resource, reflect::erased_serde::private::serde::__private::de};
use bevy_widgets::{
    widget::button::{ButtonColor, ButtonTheme, ButtonWidgetBundle, TriggerPolicy},
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

const COLOR_BACKGROUND: Color = Color::rgb(0.047, 0.109, 0.172);
const COLOR_CONTENT_BACKGROUND: Color = Color::rgb(0.065, 0.127, 0.195);
const COLOR_CONTENT_EXAMPLE: Color = Color::rgb(0.055, 0.12, 0.19);

const PRIMARY: Color = Color::rgb(0.2274, 0.2, 0.2078);
// const COLOR_CONTENT_BACKGROUND: Color = Color::hsl(224, 15, 39);
const H1_FONT_SIZE: f32 = 30.0;
const COLOR_TEXT: Color = Color::rgb(0.905, 0.921, 0.941);

const H1_FONT: &str = "fonts/Roboto/Roboto-Bold.ttf";
const TEXT_FONT: &str = "fonts/Roboto/Roboto-Regular.ttf";
const MATERIAL_FONT: &str = "fonts/MaterialIcons-Regular.ttf";


const TEXT_FONT_SIZE: f32 = 18.0;
const BUTTON_FONT_SIZE: f32 = 20.0;
const ICON_FONT_SIZE: f32 = 25.0;


const BUTTON_THEME: ButtonTheme = ButtonTheme {
    background_enabled: ButtonColor {
        pressed: Color::rgb(0.7, 0.7, 0.7),
        released: Color::rgb(0.6, 0.6, 0.6),
        hovered: Color::GRAY,
        default: Color::DARK_GRAY,
    },
    background_disabled: ButtonColor {
        pressed: Color::rgb(0.16, 0.16, 0.16),
        released: Color::rgb(0.16, 0.16, 0.16),
        hovered: Color::rgb(0.16, 0.16, 0.16),
        default: Color::rgb(0.15, 0.15, 0.15),
    },
    foreground_enabled: ButtonColor {
        pressed: Color::BLUE,
        released: Color::rgb(0.905, 0.921, 0.941),
        hovered: Color::rgb(0.905, 0.921, 0.941),
        default: Color::rgb(0.905, 0.921, 0.941),
    },
    foreground_disabled: ButtonColor {
        pressed: Color::rgb(0.35, 0.35, 0.35),
        released: Color::rgb(0.35, 0.35, 0.35),
        hovered: Color::rgb(0.35, 0.35, 0.35),
        default: Color::rgb(0.35, 0.35, 0.35),
    },
};

struct Fonts {
    h1: Handle<Font>,
    p: Handle<Font>,
    icon: Handle<Font>
}

/// Camera
fn setup_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());
}


fn setup_page(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // Fonts
    let fonts = Fonts {
        h1: asset_server.load(H1_FONT),
        p: asset_server.load(TEXT_FONT),
        icon: asset_server.load(MATERIAL_FONT),
    };

    // root node
    cmd.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            color: COLOR_BACKGROUND.into(),
            ..default()
        }).with_children(| root| {
            // Content container
            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Percent(100.0)),
                    // min_size: Size::new(Val::Px(400.0), Val::Auto),
                    // max_size: Size::new(Val::Px(800.0), Val::Auto),
                    padding: UiRect::all(Val::Px(30.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                color: COLOR_BACKGROUND.into(),
                ..default()
            }).with_children(| content | {
                // Title text
                content.spawn_bundle(TextBundle::from_section(
                    "Buttons", 
                    TextStyle {
                        font: fonts.h1.clone(),
                        font_size: H1_FONT_SIZE,
                        color: COLOR_TEXT,
                    }
                ).with_style(Style {
                     margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(15.0), Val::Undefined),
                    ..default()
                }));
                // Paragraph
                content.spawn_bundle(TextBundle::from_section(
                    "Buttons are used to trigger actions. \
                    They can be hovered and clicked. \
                    The most basic button is the text button",
                    TextStyle {
                        font: fonts.p.clone(),
                        font_size: TEXT_FONT_SIZE,
                        color: COLOR_TEXT,
                    }
                ));
                
                // Example Showcase
                content.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        
                        ..default()
                    },
                    color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {

                    // Spawn text-buttons
                    for text in ["Ok", "Submit", "Restart Game"] {
                        example_showcase.spawn_bundle(
                            ButtonWidgetBundle::new(Style {
                                size: Size::new(Val::Auto, Val::Auto),
                                padding: UiRect::new(Val::Px(25.0), Val::Px(25.0), Val::Px(10.0), Val::Px(10.0)),
                                margin: UiRect::all(Val::Px(10.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            }, 
                            BUTTON_THEME
                        )).with_children(| button | {
                            button.spawn_bundle(TextBundle::from_section(
                                text,
                                TextStyle {
                                    font: fonts.p.clone(),
                                    font_size: BUTTON_FONT_SIZE,
                                    color: COLOR_TEXT,
                                }
                            ));
                        });
                    }
                });

                // Title text
                content.spawn_bundle(TextBundle::from_section(
                    "Disabled Buttons", 
                    TextStyle {
                        font: fonts.h1.clone(),
                        font_size: H1_FONT_SIZE,
                        color: COLOR_TEXT,
                    }
                ).with_style(Style {
                    margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(15.0), Val::Undefined),
                   ..default()
               }));
                // Paragraph
                content.spawn_bundle(TextBundle::from_section(
                    "Buttons can be either enabled or disabled. \
                            Disabled buttons should not be triggered by the user. \
                            Buttons should clearly show when they are disabled by changing colors.",
                    TextStyle {
                        font: fonts.p.clone(),
                        font_size: TEXT_FONT_SIZE,
                        color: COLOR_TEXT,
                    }
                ));
                
                // Example Showcase
                content.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        
                        ..default()
                    },
                    color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {

                    // Spawn text-buttons
                    for text in ["Ok", "Submit", "Restart Game"] {
                        example_showcase.spawn_bundle(
                            ButtonWidgetBundle::new(Style {
                                size: Size::new(Val::Auto, Val::Auto),
                                padding: UiRect::new(Val::Px(25.0), Val::Px(25.0), Val::Px(10.0), Val::Px(10.0)),
                                margin: UiRect::all(Val::Px(10.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            }, 
                            BUTTON_THEME
                        ).with_enabled(false)
                        ).with_children(| button | {
                            button.spawn_bundle(TextBundle::from_section(
                                text,
                                TextStyle {
                                    font: fonts.p.clone(),
                                    font_size: BUTTON_FONT_SIZE,
                                    color: COLOR_TEXT,
                                }
                            ));
                        });
                    }
                });


                // Title text
                content.spawn_bundle(TextBundle::from_section(
                    "Trigger Policy", 
                    TextStyle {
                        font: fonts.h1.clone(),
                        font_size: H1_FONT_SIZE,
                        color: COLOR_TEXT,
                    }
                ).with_style(Style {
                    margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(15.0), Val::Undefined),
                   ..default()
               }));
                // Paragraph
                content.spawn_bundle(TextBundle::from_section(
                    "Buttons are triggered on release by default. By setting the trigger-policy you can change the button to trigger on press instead.",
                    TextStyle {
                        font: fonts.p.clone(),
                        font_size: TEXT_FONT_SIZE,
                        color: COLOR_TEXT,
                    }
                ));
                
                // Example Showcase
                content.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        
                        ..default()
                    },
                    color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {
                    // 1. Button without content
                    example_showcase.spawn_bundle(
                        ButtonWidgetBundle::new(Style {
                            size: Size::new(Val::Px(150.0), Val::Px(40.0)),
                            margin: UiRect::all(Val::Px(10.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        }, 
                        BUTTON_THEME
                    ).with_policy(TriggerPolicy::OnPress)
                    ).with_children(| button | {
                        button.spawn_bundle(TextBundle::from_section(
                            "Ok",
                            TextStyle {
                                font: fonts.p.clone(),
                                font_size: BUTTON_FONT_SIZE,
                                color: COLOR_TEXT,
                            }
                        ));
                    });

                    // 2. Button with text
                    example_showcase.spawn_bundle(
                        ButtonWidgetBundle::new(Style {
                            size: Size::new(Val::Px(150.0), Val::Px(40.0)),
                            margin: UiRect::all(Val::Px(10.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        }, 
                        BUTTON_THEME
                    ).with_policy(TriggerPolicy::OnPress)
                    ).with_children(| button | {
                        button.spawn_bundle(TextBundle::from_section(
                            "Submit",
                            TextStyle {
                                font: fonts.p.clone(),
                                font_size: BUTTON_FONT_SIZE,
                                color: COLOR_TEXT,
                            }
                        ));
                    });

                    // 3. Button with icon and text
                    example_showcase.spawn_bundle(
                        ButtonWidgetBundle::new(Style {
                            size: Size::new(Val::Px(150.0), Val::Px(40.0)),
                            margin: UiRect::all(Val::Px(10.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        }, 
                        BUTTON_THEME
                    ).with_policy(TriggerPolicy::OnPress)
                    ).with_children(| button | {
                        button.spawn_bundle(TextBundle::from_section(
                            Icon::Wifi.to_string(),
                            TextStyle {
                                font: fonts.icon.clone(),
                                font_size: ICON_FONT_SIZE,
                                color: COLOR_TEXT,
                            }
                        ));
                        button.spawn_bundle(TextBundle::from_section(
                            "Enable Wifi",
                            TextStyle {
                                font: fonts.p.clone(),
                                font_size: BUTTON_FONT_SIZE,
                                color: COLOR_TEXT,
                            }
                        ));
                    });
                });


                // Title text
                content.spawn_bundle(TextBundle::from_section(
                    "Icon Buttons", 
                    TextStyle {
                        font: fonts.h1.clone(),
                        font_size: H1_FONT_SIZE,
                        color: COLOR_TEXT,
                    }
                ).with_style(Style {
                    margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(15.0), Val::Undefined),
                   ..default()
               }));
                // Paragraph
                content.spawn_bundle(TextBundle::from_section(
                    "Some buttons only need icon.",
                    TextStyle {
                        font: fonts.p.clone(),
                        font_size: TEXT_FONT_SIZE,
                        color: COLOR_TEXT,
                    }
                ));
                
                // Example Showcase
                content.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        
                        ..default()
                    },
                    color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {

                    // Spawn icon-buttons
                    for entry in [Icon::Wifi, Icon::Subtitles, Icon::Delete, Icon::Add, Icon::Home] {
                        example_showcase.spawn_bundle(
                            ButtonWidgetBundle::new(Style {
                                size: Size::new(Val::Px(40.0), Val::Px(40.0)),
                                margin: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            }, 
                            BUTTON_THEME
                        )).with_children(| button | {
                            button.spawn_bundle(TextBundle::from_section(
                                entry.to_string(),
                                TextStyle {
                                    font: fonts.icon.clone(),
                                    font_size: ICON_FONT_SIZE,
                                    color: COLOR_TEXT,
                                })
                            );
                        });
                    }
                });

                // Title text
                content.spawn_bundle(TextBundle::from_section(
                    "Icon and text buttons", 
                    TextStyle {
                        font: fonts.h1.clone(),
                        font_size: H1_FONT_SIZE,
                        color: COLOR_TEXT,
                    }
                ).with_style(Style {
                    margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(15.0), Val::Undefined),
                   ..default()
               }));
                // Paragraph
                content.spawn_bundle(TextBundle::from_section(
                    "We can also create buttons that has both icons and text.",
                    TextStyle {
                        font: fonts.p.clone(),
                        font_size: TEXT_FONT_SIZE,
                        color: COLOR_TEXT,
                    }
                ));
                
                // Example Showcase
                content.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        
                        ..default()
                    },
                    color: COLOR_CONTENT_EXAMPLE.into(),
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
                        example_showcase.spawn_bundle(
                            ButtonWidgetBundle::new(Style {
                                size: Size::new(Val::Auto, Val::Auto),
                                padding: UiRect::new(Val::Px(15.0), Val::Px(15.0), Val::Px(10.0), Val::Px(10.0)),
                                margin: UiRect::all(Val::Px(3.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            }, 
                            BUTTON_THEME
                        )).with_children(| button | {
                            button.spawn_bundle(TextBundle::from_section(
                                icon.to_string(),
                                TextStyle {
                                    font: fonts.icon.clone(),
                                    font_size: ICON_FONT_SIZE,
                                    color: COLOR_TEXT,
                                }
                            ).with_style(Style {
                                margin: UiRect::new(Val::Undefined, Val::Px(5.0), Val::Undefined, Val::Undefined),
                                ..default()
                            }));
                            button.spawn_bundle(TextBundle::from_section(
                                text,
                                TextStyle {
                                    font: fonts.p.clone(),
                                    font_size: BUTTON_FONT_SIZE,
                                    color: COLOR_TEXT,
                                }
                            ));
                        });
                    }
                });
            });
        });
}
