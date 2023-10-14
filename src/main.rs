use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins.set(
	    WindowPlugin {
		primary_window: Some(Window {
		    title: "Bevy Game".to_string(),
		    canvas: Some("#bevy".to_owned()),
		    resolution: (800., 600.).into(),
		    prevent_default_event_handling: false,
		    ..default()
		}),
		..default()
	    }))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_test_sprite)
        .run();
}

#[derive(Component)]
pub struct Gojo;

pub fn spawn_test_sprite(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
	(
	    SpriteBundle {
		transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
		texture: asset_server.load("tile.png"),
		..default()
	    },
	    Gojo {},
	));

}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {

    let window = window_query.get_single().unwrap();

    commands.spawn(
	Camera2dBundle {
	    transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 100.0),
	    ..default()
	}
    );
}
