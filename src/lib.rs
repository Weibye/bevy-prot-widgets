use bevy::{
    prelude::{info, App, Changed, Entity, Plugin, Query, ParallelSystemDescriptorCoercion},
    ui::Interaction,
};

mod entity;
mod system;

pub use entity::*;
pub use system::*;

// Widgetplugin should be the collector of all the widget systems
pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        // Systems
        app.add_startup_system(setup_resources)
            .add_system(button_output)
            .add_system(toggle_system)
            .add_system(update_checkbox.after(toggle_system))
            .add_system(update_checkbox_color);
    }
}

struct WidgetConfig {
    // Font
    // TODO: Make it possible to add config on font style
}

// Debug systems
fn button_output(q: Query<(Entity, &Interaction), Changed<Interaction>>) {
    for (entity, interaction) in &q {
        info!("{:?} changed: {:?}", entity, interaction);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
