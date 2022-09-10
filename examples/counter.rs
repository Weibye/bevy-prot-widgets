use std::ptr::null;

use bevy::{prelude::*, reflect::erased_serde::__private::serde::__private::de};

use bevy_prot_widgets::{
    content_builder::*,
    theme::WidgetTheme,
    widget::{
        button::{ButtonColor, ButtonTheme, ButtonWidgetBundle, TriggerPolicy, ButtonEvent},
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
        .add_system(counter_interact)
        .add_system(display_counter_text.after(counter_interact))
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
    // let mut counter_widget = cmd.spawn().insert(CounterWidget);
    // let mut counter_widget: Option<CounterWidget>;

    cmd.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        color: COLOR_BACKGROUND.into(),
        ..default()
    })
    .with_children(|root| {
        // Content container
        root.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
        })
        .with_children(|content| {
            create_h1(content, &theme, "Counter");
            create_p(content, &theme, "This example showcases a simple counter. This also demonstrates how to connect buttons to functionality though the rest of your application.");

            // Example Showcase
            content
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(
                            Val::Undefined,
                            Val::Undefined,
                            Val::Px(10.0),
                            Val::Px(10.0),
                        ),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                })
                .with_children(|example_showcase| {
                    example_showcase
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Auto),
                                ..default()
                            },
                            color: theme.button_theme.background.default.clone().into(),
                            ..default()
                        })
                        .with_children(|counter_background| {
                            let add_button =
                                counter_background
                                    .spawn_bundle(
                                        ButtonWidgetBundle::new(
                                            Style {
                                                size: Size::new(Val::Auto, Val::Auto),
                                                padding: UiRect::new(
                                                    Val::Px(10.0),
                                                    Val::Px(10.0),
                                                    Val::Px(5.0),
                                                    Val::Px(5.0),
                                                ),
                                                // margin: UiRect::all(Val::Px(4.0)),
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
                                            TextBundle::from_section(
                                                Icon::Add.to_string(),
                                                theme.icon.clone(),
                                            )
                                            .with_style(Style {
                                                margin: UiRect::new(
                                                    Val::Undefined,
                                                    Val::Px(3.0),
                                                    Val::Undefined,
                                                    Val::Undefined,
                                                ),
                                                ..default()
                                            }),
                                        );
                                    })
                                    .id();

                            let counter = counter_background.spawn_bundle(
                                TextBundle::from_section("0", theme.h1).with_style(Style {
                                    // min_size: Size::new(Val::Px(50.0), Val::Auto),
                                    // margin: UiRect::new(Val::Px(50.0), Val::Px(15.0), Val::Px(10.0), Val::Px(10.0)),
                                    // margin: UiRect::new(Val::Px(15.0), Val::Px(15.0), Val::Px(10.0), Val::Px(10.0)),
                                    ..default()
                                })
                                .with_text_alignment(TextAlignment::CENTER)
                                .with_style(Style {
                                    // align_items: todo!(),
                                    align_self: AlignSelf::Center,
                                    // align_content: todo!(),
                                    // justify_content: todo!(),
                                    // margin: todo!(),
                                    margin: UiRect::new(Val::Px(15.0), Val::Px(15.0), Val::Undefined, Val::Undefined),
                                    ..default()
                                }),
                            ).insert(Counter(0)).id();

                            let remove_button =
                            counter_background
                                    .spawn_bundle(
                                        ButtonWidgetBundle::new(
                                            Style {
                                                size: Size::new(Val::Auto, Val::Auto),
                                                padding: UiRect::new(
                                                    Val::Px(10.0),
                                                    Val::Px(10.0),
                                                    Val::Px(5.0),
                                                    Val::Px(5.0),
                                                ),
                                                // margin: UiRect::all(Val::Px(4.0)),
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
                                            TextBundle::from_section(
                                                Icon::Remove.to_string(),
                                                theme.icon.clone(),
                                            )
                                            .with_style(Style {
                                                margin: UiRect::new(
                                                    Val::Undefined,
                                                    Val::Px(3.0),
                                                    Val::Undefined,
                                                    Val::Undefined,
                                                ),
                                                ..default()
                                            }),
                                        );
                                    })
                                    .id();
                            
                            // counter_background.spawn().insert(CounterWidget {
                            //     count: 0,
                            //     increment_button: add_button,
                            //     decrement_button: remove_button,
                            //     display_text: counter,
                            // });

                            // counter_widget = Some();
                        });

                    // cmd.entity(counter).insert(CounterWidget { count: 0, increase: add_button, decrease: remove_button });
                });
        });
    });

    // if let Some(value) = counter_widget {
    //     cmd.spawn().insert(value);
    // }
}


#[derive(Component)]
struct Counter(pub i32);

#[derive(Component)]
pub struct CounterWidget {
    pub increment_button: Entity,
    pub decrement_button: Entity,
    pub display_text: Entity,
}

pub(crate) fn counter_interact(
    mut q: Query<&mut CounterWidget>,
    mut reader: EventReader<ButtonEvent>,
) {
    for event in reader.iter() {
        for mut counter in &mut q {
            counter.count += if counter.increment_button == event.0 {
                1
            } else if counter.decrement_button == event.0 {
                -1
            } else {
                0
            };
        }
    }
}

/// Update the text whenever the counter changes
pub(crate) fn display_counter_text(mut q_text: Query<&mut Text>, q_counter: Query<&CounterWidget, Changed<CounterWidget>>) {
    for widget in &q_counter {
        if let Ok(mut display) = q_text.get_mut(widget.display_text) {
            display.sections[0].value = widget.count.to_string();
        }
    }
}
