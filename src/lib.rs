use bevy::{
    prelude::{info, App, Changed, Entity, Plugin, Query},
    ui::Interaction,
};

mod entity;
mod system;

pub use entity::*;
pub use system::*;

// Widgetplugin should be the collector of all the widget systems
struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        // Systems
        // app.add_system(button_system)
        app.add_system(button_output).add_system(toggle_system);
    }
}

// Debug systems

fn button_output(q: Query<(Entity, &Interaction), Changed<Interaction>>) {
    for (entity, interaction) in &q {
        info!("{:?} changed: {:?}", entity, interaction);
    }
}

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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
