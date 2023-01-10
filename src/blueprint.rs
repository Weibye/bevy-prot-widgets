use bevy::{prelude::Commands, ecs::system::EntityCommands};


pub trait WidgetBlueprint<'w, 's> {
    fn build<'a>(self, cmd: &'w mut EntityCommands<'w, 's, 'a>) -> &'w mut EntityCommands<'w, 's, 'a>;
}