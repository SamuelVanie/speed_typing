use bevy::{prelude::*, window::PrimaryWindow};

mod text;
mod decor;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
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
        .add_plugins(text::TextPlugin)
        .run();
}
