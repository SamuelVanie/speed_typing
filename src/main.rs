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
        .run();
}

pub struct TextElements {
    text: String,
    typed_text: String
}

#[derive(Component)]
// Text that should be displayed, it should have two sections, one for the text that is typed and one for the text that is not typed
pub struct CurrentText(Text); 


pub enum TextColor {
    RED,
    GREEN,
    WHITE
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

