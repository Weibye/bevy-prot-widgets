use bevy::{
    prelude::{AssetServer, Commands, Handle, Res, Resource},
    text::Font,
};

const MATERIAL_FONT: &str = "fonts/MaterialIcons-Regular.ttf";

pub(crate) fn load_fonts(mut cmd: Commands, asset_loader: Res<AssetServer>) {
    cmd.insert_resource(FontLib {
        material: asset_loader.load(MATERIAL_FONT),
    });
}

#[derive(Resource)]
pub struct FontLib {
    pub material: Handle<Font>,
}
