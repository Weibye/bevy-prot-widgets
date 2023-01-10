use bevy::{
    prelude::{info, App, Changed, Entity, IntoSystemDescriptor, Plugin, Query, StartupStage},
    ui::Interaction,
};
use fonts::load_fonts;
use system::{setup_resources, toggle_system, update_widget_colors};

pub mod blueprint;
pub mod content_builder;
mod entity;
pub mod fonts;
mod system;
pub mod theme;
pub mod widget;

pub use entity::*;
pub use system::*;

use widget::{
    button::{button_color, button_interaction, button_trigger, on_button_trigger, ButtonEvent},
    checkbox::{update_checkbox_icon, update_checkbox_interaction},
    icon::update_changed_icons,
    radio::{update_radio_icon, update_radio_interaction},
};

// Widgetplugin should be the collector of all the widget systems
pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        // Systems
        app.add_event::<ButtonEvent>()
            .add_startup_system_to_stage(StartupStage::PreStartup, load_fonts)
            .add_startup_system(setup_resources)
            .add_system(update_radio_interaction)
            .add_system(
                update_radio_icon
                    .before(update_changed_icons)
                    .after(toggle_system),
            )
            .add_system(update_checkbox_interaction)
            .add_system(
                update_checkbox_icon
                    .before(update_changed_icons)
                    .after(update_checkbox_interaction),
            )
            .add_system(update_changed_icons)
            .add_system(button_output)
            .add_system(toggle_system)
            .add_system(button_color)
            .add_system(button_interaction)
            .add_system(button_trigger.after(button_interaction))
            .add_system(on_button_trigger.after(button_trigger))
            // .add_system(update_checkbox.after(toggle_system))
            .add_system(update_widget_colors);
        // Load the correct fonts and put in a resource
    }
}

// Debug systems
fn button_output(q: Query<(Entity, &Interaction), Changed<Interaction>>) {
    for (entity, interaction) in &q {
        info!("{:?} changed: {:?}", entity, interaction);
    }
}

// #[derive(Resource)]

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
