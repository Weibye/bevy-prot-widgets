use bevy::{
    prelude::{AssetServer, Commands, Handle, Res, Resource},
    text::Font,
};

const MATERIAL_FONT: &str = "fonts/MaterialIcons-Regular.ttf";
const NORMAL_FONT: &str = "fonts/Roboto/Roboto-Regular.ttf";

pub(crate) fn load_fonts(mut cmd: Commands, asset_loader: Res<AssetServer>) {
    cmd.insert_resource(FontLib {
        material: asset_loader.load(MATERIAL_FONT),
        normal: asset_loader.load(NORMAL_FONT),
    });
}

#[derive(Resource)]
pub struct FontLib {
    pub material: Handle<Font>,
    pub normal: Handle<Font>,
}
