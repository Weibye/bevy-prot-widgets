use bevy::prelude::*;

use bevy_prot_widgets::{
    content_builder::*,
    theme::WidgetTheme,
    widget::{button::{ButtonColor, ButtonTheme, TriggerPolicy, ButtonWidgetBundle}, counter::CounterWidget},
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
    cmd.spawn_bundle(Camera2dBundle::default());
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

            create_h1(content, &theme, "Counter");
            
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
                // create_label_button(example_showcase, &theme, Icon::Add, "Add", true, TriggerPolicy::OnRelease);
                let add_button = example_showcase
                    .spawn_bundle(
                        ButtonWidgetBundle::new(
                            Style {
                                size: Size::new(Val::Auto, Val::Auto),
                                padding: UiRect::new(
                                    Val::Px(15.0),
                                    Val::Px(15.0),
                                    Val::Px(10.0),
                                    Val::Px(10.0),
                                ),
                                margin: UiRect::all(Val::Px(4.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            theme.button_theme.clone(),
                        )
                        .with_enabled(true)
                        .with_policy(TriggerPolicy::OnRelease),
                    )
                    .with_children(|button| {
                        button.spawn_bundle(
                            TextBundle::from_section(Icon::Add.to_string(), theme.icon.clone()).with_style(Style {
                                margin: UiRect::new(
                                    Val::Undefined,
                                    Val::Px(3.0),
                                    Val::Undefined,
                                    Val::Undefined,
                                ),
                                ..default()
                            }),
                        );
                    }).id();
                
                let counter = example_showcase
                    .spawn_bundle(TextBundle::from_section("0", theme.h1).with_style(Style {
                        min_size: Size::new(Val::Px(30.0), Val::Auto),
                        ..default()
                    })).id();
                
                let remove_button = example_showcase
                    .spawn_bundle(
                        ButtonWidgetBundle::new(
                            Style {
                                size: Size::new(Val::Auto, Val::Auto),
                                padding: UiRect::new(
                                    Val::Px(15.0),
                                    Val::Px(15.0),
                                    Val::Px(10.0),
                                    Val::Px(10.0),
                                ),
                                margin: UiRect::all(Val::Px(4.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            theme.button_theme.clone(),
                        )
                        .with_enabled(true)
                        .with_policy(TriggerPolicy::OnRelease),
                    )
                    .with_children(| button| {
                        button.spawn_bundle(
                            TextBundle::from_section(Icon::Remove.to_string(), theme.icon.clone()).with_style(Style {
                                margin: UiRect::new(
                                    Val::Undefined,
                                    Val::Px(3.0),
                                    Val::Undefined,
                                    Val::Undefined,
                                ),
                                ..default()
                            }),
                        );
                    }).id();

                cmd.entity(counter).insert(CounterWidget { count: 0, increase: add_button, decrease: remove_button });
            });
        });
    });
}



// WHen button is pressed, it triggers an event
// That event contains the entity of the button it was attached to
// Somehow define

