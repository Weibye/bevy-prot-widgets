use bevy::prelude::{Bundle, NodeBundle, Component};

#[derive(Component)]
pub struct LabelWidget {
    
}

#[derive(Bundle)]
pub struct LabelWidgetBundle {
    #[bundle]
    pub node_bundle: NodeBundle,

}