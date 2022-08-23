use bevy::{
    prelude::{info, AssetServer, Changed, Query, Res, With, Color, Commands},
    text::{Text, TextStyle},
    ui::Interaction,
};

use crate::{
    entity::{CheckboxWidget, ToggleState},
    CheckboxIcons,
};

const MATERIAL_FONT: &str = "fonts/MaterialIcons-Regular.ttf";
const ICON_SIZE: f32 = 40.0;
const ICON_COLOR: Color = Color::DARK_GRAY;

const ICON_COLOR_NORMAL: Color = Color::DARK_GRAY;
const ICON_COLOR_HOVERED: Color = Color::rgb(0.5, 0.5, 0.5);
const ICON_COLOR_SELECTED: Color = Color::rgb_linear(0.3, 0.3, 0.7);

// #[derive(Resource)]
pub struct IconStyle(pub TextStyle);

pub(crate) fn setup_resources(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.insert_resource(IconStyle(TextStyle {
        font: asset_server.load(MATERIAL_FONT),
        font_size: ICON_SIZE,
        color: ICON_COLOR,
    }));
}

/// System responsible for toggling the state of widgets that can toggle
pub(crate) fn toggle_system(mut q: Query<(&mut ToggleState, &Interaction), Changed<Interaction>>) {
    for (mut state, interaction) in &mut q {
        match *interaction {
            Interaction::Clicked => {
                state.0 = !state.0;
                info!("Toggled state to {:?}", state.0);
            }
            _ => {}
        }
    }
}

/// System that updates the visual of the checkbox according to their state
pub(crate) fn update_checkbox(
    mut q: Query<(&mut Text, &ToggleState, &CheckboxIcons), (Changed<ToggleState>, With<CheckboxWidget>)>,
) {
    for (mut text, state, icons) in &mut q {
        // Assume only one section in widgets for now
        text.sections[0].value = (if state.0 { icons.checked } else { icons.empty }).to_string();
    }
}

pub(crate) fn update_checkbox_color(
    mut q: Query<(&mut Text, &Interaction), (Changed<Interaction>, With<CheckboxWidget>)>
) {
    for (mut text, interaction) in &mut q {
        // Assume only one section in widgets for now
        text.sections[0].style.color = match interaction {
            Interaction::Clicked => ICON_COLOR_SELECTED,
            Interaction::Hovered => ICON_COLOR_HOVERED,
            Interaction::None => ICON_COLOR_NORMAL,
        }
    }
}
