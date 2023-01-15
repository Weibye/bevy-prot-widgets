use bevy_app::{App, Plugin, StartupStage};
use bevy_ecs::{
    prelude::Entity,
    query::Changed,
    schedule::{StageLabel, SystemStage},
    system::Query,
};
use bevy_log::info;
use bevy_ui::Interaction;

use fonts::FontPlugin;

mod entity;
pub mod fonts;
mod system;
pub mod theme;
pub mod widget;

pub use entity::*;
pub use system::*;

use theme::ThemePlugin;
use widget::{checkbox::CheckBoxPlugin, icon::IconPlugin, label::LabelPlugin, radio::RadioPlugin};

// Widgetplugin should be the collector of all the widget systems
pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        // Systems
        app.add_startup_stage_before(
            StartupStage::PreStartup,
            WidgetStage::AssetLoad,
            SystemStage::parallel(),
        )
        .add_startup_stage_before(
            StartupStage::PreStartup,
            WidgetStage::ThemeSetup,
            SystemStage::parallel(),
        )
        .add_plugin(FontPlugin)
        .add_plugin(ThemePlugin)
        .add_plugin(IconPlugin)
        .add_plugin(LabelPlugin)
        .add_plugin(RadioPlugin)
        .add_plugin(CheckBoxPlugin)
        // .add_event::<ButtonEvent>()
        .add_system(button_output);
        // .add_system(toggle_system)
        // .add_system(button_color)
        // .add_system(button_interaction)
        // .add_system(button_trigger.after(button_interaction))
        // .add_system(on_button_trigger.after(button_trigger))
        // .add_system(update_checkbox.after(toggle_system))
        // .add_system(update_widget_colors);
        // Load the correct fonts and put in a resource
    }
}

#[derive(StageLabel)]
pub enum WidgetStage {
    AssetLoad,
    ThemeSetup,
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
