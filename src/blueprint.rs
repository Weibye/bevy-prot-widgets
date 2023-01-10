use bevy::ecs::system::EntityCommands;

pub trait WidgetBlueprint<'w, 's> {
    fn build<'a>(
        self,
        cmd: &'a mut EntityCommands<'w, 's, 'a>,
    ) -> &'a mut EntityCommands<'w, 's, 'a>;
}
