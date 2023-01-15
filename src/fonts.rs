use bevy_app::{App, Plugin};
use bevy_asset::{AssetServer, Handle};
use bevy_ecs::system::{Commands, Res, Resource};
use bevy_text::Font;

use crate::WidgetStage;

const MATERIAL_FONT: &str = "fonts/MaterialIcons-Regular.ttf";
const H1_FONT: &str = "fonts/Roboto/Roboto-Regular.ttf";
const H2_FONT: &str = "fonts/Roboto/Roboto-Regular.ttf";
const H3_FONT: &str = "fonts/Roboto/Roboto-Regular.ttf";
const PARAGRAPH_FONT: &str = "fonts/Roboto/Roboto-Regular.ttf";

pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(WidgetStage::AssetLoad, load_fonts);
    }
}

pub(crate) fn load_fonts(mut cmd: Commands, asset_loader: Res<AssetServer>) {
    cmd.insert_resource(FontHandles {
        material: asset_loader.load(MATERIAL_FONT),
        h1: asset_loader.load(H1_FONT),
        h2: asset_loader.load(H2_FONT),
        h3: asset_loader.load(H3_FONT),
        p: asset_loader.load(PARAGRAPH_FONT),
    });
}

#[derive(Resource)]
pub struct FontPaths {
    pub material: Handle<Font>,
    pub h1: Handle<Font>,
    pub h2: Handle<Font>,
    pub h3: Handle<Font>,
    pub p: Handle<Font>,
}

#[derive(Resource)]
pub struct FontHandles {
    pub material: Handle<Font>,
    pub h1: Handle<Font>,
    pub h2: Handle<Font>,
    pub h3: Handle<Font>,
    pub p: Handle<Font>,
}
