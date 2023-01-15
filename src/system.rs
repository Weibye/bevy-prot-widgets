// use bevy_ecs::system::Resource;
// use bevy_log::info;
// use bevy_render::color::Color;
// use bevy_text::TextStyle;
// use bevy_ui::Interaction;

// use crate::entity::ToggleState;

// const MATERIAL_FONT: &str = "fonts/MaterialIcons-Regular.ttf";
// const ICON_SIZE: f32 = 40.0;
// const ICON_COLOR: Color = Color::DARK_GRAY;

// const ICON_COLOR_NORMAL: Color = Color::DARK_GRAY;
// const ICON_COLOR_HOVERED: Color = Color::rgb(0.5, 0.5, 0.5);
// const ICON_COLOR_SELECTED: Color = Color::rgb_linear(1.0, 0.3, 0.2);

// /// System responsible for toggling the state of widgets that can toggle
// pub(crate) fn toggle_system(mut q: Query<(&mut ToggleState, &Interaction), Changed<Interaction>>) {
//     for (mut state, interaction) in &mut q {
//         if *interaction == Interaction::Clicked {
//             state.0 = !state.0;
//             info!("Toggled state to {:?}", state.0);
//         }
//     }
// }

// /// System that updates the visual of the checkbox according to their state
// pub(crate) fn update_checkbox(mut q: Query<CheckboxQuery, CheckboxChanged>) {
//     for mut checkbox in &mut q {
//         // Assume only one section in widgets for now
//         checkbox.text.sections[0].value = match checkbox.state {
//             CheckboxState::Checked => checkbox.icons.checked,
//             CheckboxState::Unchecked => checkbox.icons.unchecked,
//             CheckboxState::Indeterminate => checkbox.icons.indeterminate,
//         }
//         .to_string();
//     }
// }

// pub(crate) fn update_widget_colors(mut q: Query<(&mut Text, &Interaction), WidgetChanged>) {
//     for (mut text, interaction) in &mut q {
//         // Assume only one section in widgets for now
//         text.sections[0].style.color = match interaction {
//             Interaction::Clicked => ICON_COLOR_SELECTED,
//             Interaction::Hovered => ICON_COLOR_HOVERED,
//             Interaction::None => ICON_COLOR_NORMAL,
//         }
//     }
// }

// pub(crate) fn update_checkbox_colors(
//     mut q: Query<(&mut Text, &Interaction), (Changed<Interaction>, With<CheckboxWidget>)>,
// ) {
//     for (mut text, interaction) in &mut q {
//         // Assume only one section in widgets for now
//         text.sections[0].style.color = match interaction {
//             Interaction::Clicked => ICON_COLOR_SELECTED,
//             Interaction::Hovered => ICON_COLOR_HOVERED,
//             Interaction::None => ICON_COLOR_NORMAL,
//         }
//     }
// }
