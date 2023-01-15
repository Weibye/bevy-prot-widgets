use crate::{fonts::FontHandles, WidgetStage};
use bevy_app::{App, Plugin};
use bevy_ecs::system::{Commands, Res, Resource};
use bevy_render::color::Color;
use bevy_text::TextStyle;

const COLOR_BACKGROUND: Color = Color::rgb(0.047, 0.109, 0.172);
// const COLOR_CONTENT_EXAMPLE: Color = Color::rgb(0.055, 0.12, 0.19);
const COLOR_TEXT: Color = Color::rgb(0.905, 0.921, 0.941);

const H1_FONT_SIZE: f32 = 35.0;
const H2_FONT_SIZE: f32 = 30.0;
const H3_FONT_SIZE: f32 = 25.0;
const PARAGRAPH_FONT_SIZE: f32 = 18.0;
const BUTTON_FONT_SIZE: f32 = 20.0;
const ICON_FONT_SIZE: f32 = 25.0;

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        // When fonts are loaded, create the widget theme resource
        app.add_startup_system_to_stage(WidgetStage::ThemeSetup, setup_theme);
    }
}

fn setup_theme(mut cmd: Commands, fonts: Res<FontHandles>) {
    cmd.insert_resource(WidgetTheme {
        background_color: COLOR_BACKGROUND,
        h1: TextStyle {
            font: fonts.h1.clone(),
            font_size: H1_FONT_SIZE,
            color: COLOR_TEXT,
        },
        h2: TextStyle {
            font: fonts.h2.clone(),
            font_size: H2_FONT_SIZE,
            color: COLOR_TEXT,
        },
        h3: TextStyle {
            font: fonts.h3.clone(),
            font_size: H3_FONT_SIZE,
            color: COLOR_TEXT,
        },
        p: TextStyle {
            font: fonts.p.clone(),
            font_size: PARAGRAPH_FONT_SIZE,
            color: COLOR_TEXT,
        },
        icon: TextStyle {
            font: fonts.material.clone(),
            font_size: ICON_FONT_SIZE,
            color: COLOR_TEXT,
        },
        button: TextStyle {
            font: fonts.p.clone(), 
            font_size: BUTTON_FONT_SIZE,
            color: COLOR_TEXT
        },
    });
}

// This is a stopgap solution until a more complete and sensible theming-system can be figured out
#[derive(Resource)]
pub struct WidgetTheme {
    // Whatever fonts we need
    pub background_color: Color,
    pub h1: TextStyle,
    pub h2: TextStyle,
    pub h3: TextStyle,
    pub p: TextStyle,
    pub icon: TextStyle,
    pub button: TextStyle,
    // Whatever colors we need
    // pub button_theme: ButtonTheme,
}
