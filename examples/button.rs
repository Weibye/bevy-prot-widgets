use bevy::prelude::*;

use bevy_prot_widgets::{
    content_builder::*,
    theme::WidgetTheme,
    widget::button::{ButtonColor, ButtonTheme, TriggerPolicy},
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
const COLOR_CONTENT_EXAMPLE: Color = Color::rgb(0.055, 0.12, 0.19);
const COLOR_TEXT: Color = Color::rgb(0.905, 0.921, 0.941);
const H1_FONT: &str = "fonts/Roboto/Roboto-Bold.ttf";
const TEXT_FONT: &str = "fonts/Roboto/Roboto-Regular.ttf";
const WIDGET_FONT: &str = "fonts/Roboto/Roboto-Regular.ttf";
const MATERIAL_FONT: &str = "fonts/MaterialIcons-Regular.ttf";

const H1_FONT_SIZE: f32 = 30.0;
const TEXT_FONT_SIZE: f32 = 18.0;
const BUTTON_FONT_SIZE: f32 = 20.0;
const ICON_FONT_SIZE: f32 = 20.0;

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

fn setup_page(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // Setup theme used for these widgets
    let theme = WidgetTheme {
        h1: TextStyle {
            font: asset_server.load(H1_FONT),
            font_size: H1_FONT_SIZE,
            color: COLOR_TEXT,
        },
        p: TextStyle {
            font: asset_server.load(TEXT_FONT),
            font_size: TEXT_FONT_SIZE,
            color: COLOR_TEXT,
        },
        icon: TextStyle {
            font: asset_server.load(MATERIAL_FONT),
            font_size: ICON_FONT_SIZE,
            color: COLOR_TEXT,
        },
        widget: TextStyle {
            font: asset_server.load(WIDGET_FONT),
            font_size: BUTTON_FONT_SIZE,
            color: COLOR_TEXT,
        },
        button_theme: BUTTON_THEME,
    };

    // root node
    cmd.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: COLOR_BACKGROUND.into(),
            ..default()
        }).with_children(| root| {
            // Content container
            root.spawn(NodeBundle {
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
                background_color: COLOR_BACKGROUND.into(),
                ..default()
            }).with_children(| content | {

                create_h1(content, &theme, "Buttons");
                create_p(content, &theme, "Buttons are used to trigger actions. \
                They can be hovered and clicked. The most basic button is the text button");

                // Example Showcase
                content.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {

                    create_text_button(example_showcase, &theme, "Open Inventory", true, TriggerPolicy::OnRelease);
                    create_icon_button(example_showcase, &theme, Icon::Menu, true, TriggerPolicy::OnRelease);
                    create_label_button(example_showcase, &theme, Icon::Send, "Send", true, TriggerPolicy::OnRelease);
                });


                create_h1(content, &theme, "Disabled Buttons");

                create_p(content, &theme, "Buttons can be either enabled or disabled. \
                Disabled buttons should not be triggered by the user. \
                Buttons should clearly show when they are disabled by changing colors.");
                // Example Showcase
                content.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
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
                        create_text_button(example_showcase, &theme, text, false, TriggerPolicy::OnRelease);
                    }
                });

                create_h1(content, &theme, "Trigger Policy");
                create_p(content, &theme, "Buttons are triggered on release by default. \
                By setting the trigger-policy you can change the button to trigger on press instead.");
                // Example Showcase
                content.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {
                    create_text_button(example_showcase, &theme, "Ok", true, TriggerPolicy::OnPress);
                    create_text_button(example_showcase, &theme, "Ok", true, TriggerPolicy::OnPress);
                    create_label_button(example_showcase, &theme, Icon::Wifi, "Enable Wifi", true, TriggerPolicy::OnPress);

                });

                create_h1(content, &theme, "Icon Buttons");
                create_p(content, &theme, "Some buttons only need icon.");

                // Example Showcase
                content.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
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
                        create_icon_button(example_showcase, &theme, icon, true, TriggerPolicy::OnRelease);
                    }
                });

                create_h1(content, &theme, "Icon and Text Buttons");
                create_p(content, &theme, "We can also create buttons that has both icons and text.");

                // Example Showcase
                content.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
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
                        create_label_button(example_showcase, &theme, icon, text,  true, TriggerPolicy::OnRelease);
                    }
                });
            });
        });
}
