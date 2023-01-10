use bevy::prelude::{Bundle, Component, NodeBundle};

#[derive(Component)]
pub struct LabelWidget {
    
}

#[derive(Bundle)]
pub struct LabelWidgetBundle {
    #[bundle]
    pub node_bundle: NodeBundle,
}
