use bevy::{
    prelude::{info, AssetServer, Changed, Color, Commands, Or, Query, Res, With},
    text::{Text, TextStyle},
    ui::Interaction,
};

use crate::{
    entity::ToggleState,
    widget::{
        checkbox::{CheckboxIcons, CheckboxState, CheckboxWidget},
        radio::{RadioButtonIcons, RadioButtonWidget},
    },
};

const MATERIAL_FONT: &str = "fonts/MaterialIcons-Regular.ttf";
const ICON_SIZE: f32 = 40.0;
const ICON_COLOR: Color = Color::DARK_GRAY;

const ICON_COLOR_NORMAL: Color = Color::DARK_GRAY;
const ICON_COLOR_HOVERED: Color = Color::rgb(0.5, 0.5, 0.5);
const ICON_COLOR_SELECTED: Color = Color::rgb_linear(1.0, 0.3, 0.2);

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
    mut q: Query<
        (&mut Text, &CheckboxState, &CheckboxIcons),
        (Changed<CheckboxState>, With<CheckboxWidget>),
    >,
) {
    for (mut text, state, icons) in &mut q {
        // Assume only one section in widgets for now
        text.sections[0].value = match state {
            CheckboxState::Checked => icons.checked,
            CheckboxState::Unchecked => icons.unchecked,
            CheckboxState::Indeterminate => icons.indeterminate,
        }
        .to_string();
    }
}

/// System that updates the visual of the checkbox according to their state
pub(crate) fn update_radio(
    mut q: Query<
        (&mut Text, &ToggleState, &RadioButtonIcons),
        (Changed<ToggleState>, With<RadioButtonWidget>),
    >,
) {
    for (mut text, state, icons) in &mut q {
        // Assume only one section in widgets for now
        text.sections[0].value = (if state.0 { icons.checked } else { icons.empty }).to_string();
    }
}

pub(crate) fn update_widget_colors(
    mut q: Query<
        (&mut Text, &Interaction),
        (
            Changed<Interaction>,
            Or<(With<CheckboxWidget>, With<RadioButtonWidget>)>,
        ),
    >,
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

pub(crate) fn update_checkbox_colors(
    mut q: Query<(&mut Text, &Interaction), (Changed<Interaction>, With<CheckboxWidget>)>,
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
