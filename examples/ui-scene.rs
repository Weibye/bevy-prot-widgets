//! This example illustrates loading a UI tree from a file

use std::{fs::File, io::Write};

use bevy::prelude::*;
use bevy_prot_widgets::WidgetPlugin;
use material_icons::Icon;

const MATERIAL_FONT: &str = "fonts/MaterialIcons-Regular.ttf";
const UI_SCENE: &str = "D:\\Programming\\bevy-prot-widgets\\assets\\UI.scn.ron";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WidgetPlugin)
        .add_event::<SaveEvent>()
        // .add_startup_system(load_ui_system)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_ui)
        .add_system(trigger_save)
        .add_system(save_ui_to_file.exclusive_system())
        .run();
}

struct SaveEvent;

/// Camera
fn setup_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());
}

/// Creates the UI
fn setup_ui(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // root node
    cmd.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: Color::WHITE.into(),
        ..default()
    })
    .with_children(|root| {
        root.spawn_bundle(TextBundle::from_section(
            Icon::CheckBox.to_string(),
            TextStyle {
                font: asset_server.load(MATERIAL_FONT),
                font_size: 50.0,
                color: Color::BLACK,
            },
        ));

        root.spawn_bundle(TextBundle::from_section(
            Icon::CheckBoxOutlineBlank.to_string(),
            TextStyle {
                font: asset_server.load(MATERIAL_FONT),
                font_size: 50.0,
                color: Color::BLACK,
            },
        ));

        root.spawn_bundle(TextBundle::from_section(
            Icon::Check.to_string(),
            TextStyle {
                font: asset_server.load(MATERIAL_FONT),
                font_size: 50.0,
                color: Color::BLACK,
            },
        ));

        root.spawn_bundle(TextBundle::from_section(
            Icon::CheckCircle.to_string(),
            TextStyle {
                font: asset_server.load(MATERIAL_FONT),
                font_size: 50.0,
                color: Color::BLACK,
            },
        ));
    });
}

fn trigger_save(input: Res<Input<KeyCode>>, mut event: EventWriter<SaveEvent>) {
    if input.just_pressed(KeyCode::Space) {
        info!("Sending event");
        event.send(SaveEvent);
    }
}

// fn load_ui_system(mut cmd: Commands, asset_server: Res<AssetServer>) {
//     // "Spawning" a scene bundle creates a new entity and spawns new instances
//     // of the given scene's entities as children of that entity.
//     cmd.spawn_bundle(DynamicSceneBundle {
//         // Scenes are loaded just like any other asset.
//         scene: asset_server.load("scenes/ui.scn.ron"),
//         ..default()
//     });

//     // This tells the AssetServer to watch for changes to assets.
//     // It enables our scenes to automatically reload in game when we modify their files
//     asset_server.watch_for_changes().unwrap();
// }

fn save_ui_to_file(world: &mut World) {
    let mut cell = world.cell();
    let mut reader = &cell.get_resource_mut::<Events<SaveEvent>>();
    let mut asset_server = &cell.get_resource_mut::<AssetServer>();
    if let (Some(reader), Some(asset_server)) = (reader, asset_server) {
        
        if reader.is_empty() { return; }
        
        // Scenes can be created from any ECS World. You can either create a new one for the scene or
        // use the current World.
        let mut scene_world = World::new();

        scene_world.spawn().insert_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::WHITE.into(),
            ..default()
        })
        .with_children(|root| {
            root.spawn_bundle(TextBundle::from_section(
                Icon::CheckBox.to_string(),
                TextStyle {
                    font: asset_server.load(MATERIAL_FONT),
                    font_size: 50.0,
                    color: Color::BLACK,
                },
            ));
    
            root.spawn_bundle(TextBundle::from_section(
                Icon::CheckBoxOutlineBlank.to_string(),
                TextStyle {
                    font: asset_server.load(MATERIAL_FONT),
                    font_size: 50.0,
                    color: Color::BLACK,
                },
            ));
    
            root.spawn_bundle(TextBundle::from_section(
                Icon::Check.to_string(),
                TextStyle {
                    font: asset_server.load(MATERIAL_FONT),
                    font_size: 50.0,
                    color: Color::BLACK,
                },
            ));
    
            root.spawn_bundle(TextBundle::from_section(
                Icon::CheckCircle.to_string(),
                TextStyle {
                    font: asset_server.load(MATERIAL_FONT),
                    font_size: 50.0,
                    color: Color::BLACK,
                },
            ));
        });

        // The TypeRegistry resource contains information about all registered types (including
        // components). This is used to construct scenes.
        let type_registry = &cell.resource::<AppTypeRegistry>();
        let scene = DynamicScene::from_world(&scene_world, type_registry);

        // Scenes can be serialized like this:
        let data = scene.serialize_ron(type_registry).unwrap();
        // info!("{}", scene.serialize_ron(type_registry).unwrap());
        if let Ok(mut file) = File::create(UI_SCENE) {
            file.write_all(data.as_bytes());
        }
        // file.

    }
}