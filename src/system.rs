use bevy::{
    prelude::{info, AssetServer, Button, Changed, Children, Entity, Query, Res, ResMut, With},
    ui::{Interaction, UiColor, UiImage},
};

use crate::entity::{CheckboxWidget, ToggleState};

// fn load_icons(mut icons: ResMut<Icons>, asset_server: Res<AssetServer>) {
//     icons.checkbox = asset_server.load(CHECKBOX_EMPTY);
//     icons.checkbox_checked = asset_server.load(CHECKBOX_CHECKED);
// }

// fn button_system(
//     mut query: Query<(&Interaction, &mut UiColor, &Children),(Changed<Interaction>, With<Button>)>,
//     mut text_query: Query<&mut Text>,
// ) {
//     for (interaction, mut color, children) in &mut query {
//         let mut text = text_query.get_mut(children[0]).unwrap();
//         match *interaction {
//             Interaction::Clicked => {
//                 text.sections[0].value = "Press".to_string();
//                 *color = PRESSED_BUTTON.into();
//             }
//             Interaction::Hovered => {
//                 text.sections[0].value = "Hover".to_string();
//                 *color = HOVERED_BUTTON.into();
//             }
//             Interaction::None => {
//                 text.sections[0].value = "Button".to_string();
//                 *color = NORMAL_BUTTON.into();
//             }
//         }
//     }
// }

/// System responsible for toggling the state of buttons that can toggle
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

// fn update_checkbox(
//     mut q: Query<(&mut UiImage, &ToggleState), (Changed<ToggleState>, With<CheckboxWidget>)>,
//     icons: Res<Icons>,
// ) {
//     for (mut image, state) in &mut q {
//         image.0 = if state.0 {
//             icons.checkbox_checked.unwrap()
//         } else {
//             icons.checkbox.unwrap()
//         }
//     }
// }
