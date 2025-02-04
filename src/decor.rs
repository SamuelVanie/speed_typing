use bevy::prelude::*;


fn set_background_img(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let background_img: Handle<Texture> = asset_server.load("background.png");
}


