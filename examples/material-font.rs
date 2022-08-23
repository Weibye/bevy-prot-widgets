use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(setup)
        .run();
}


const ROBOTO_MEDIUM: &str = "fonts/Roboto/Roboto-Medium.ttf";
const ICON: &str = "icons/black/donut_large/round-4x.png";

fn setup_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());
}

fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // root node
    cmd
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgb(0.5, 0.5, 0.5).into(),
            ..default()
        })
        .with_children(|root| {
            root.spawn_bundle(TextBundle::from_section(
                "Button",
                TextStyle {
                    font: asset_server.load(ROBOTO_MEDIUM),
                    font_size: 50.0,
                    color: Color::BLACK,
                },
            ));

            root.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(40.0), Val::Px(40.0)),
                    ..default()
                },
                // color: todo!(),
                image: asset_server.load(ICON).into(),
                ..default()
            });


        });
}
