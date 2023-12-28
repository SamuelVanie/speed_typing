use bevy::{prelude::*, window::PrimaryWindow};

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
        .add_plugins(TextPlugin)
        .run();
}

pub struct TextPlugin;

const FONT_SIZE: f32 = 30.0;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenText>()
            .init_resource::<CurrentCharIndex>()
            .add_systems(Startup, setup_text)
            .add_systems(Update, update_text)
            .add_systems(Update, draw_text);
    }
}

#[derive(Component)]
pub struct TextElement {
    text: char,
    color: Color,
}

pub enum CharColor {
    CORRECT,
    WRONG,
    DEFAULT,
}

pub fn get_color(color: CharColor) -> Color {
    match color {
        CharColor::CORRECT => Color::GREEN,
        CharColor::WRONG => Color::RED,
        CharColor::DEFAULT => Color::WHITE,
    }
}

#[derive(Resource)]
pub struct ScreenText(Vec<TextElement>);

impl Default for ScreenText {
    fn default() -> Self {
        ScreenText(Vec::new())
    }
}

#[derive(Resource)]
pub struct CurrentCharIndex(usize);

impl Default for CurrentCharIndex {
    fn default() -> Self {
        CurrentCharIndex(0)
    }
}

pub fn setup_text(mut screen_txt: ResMut<ScreenText>) {
    let text = "Hello World!";
    for c in text.chars() {
        screen_txt.0.push(TextElement {
            text: c,
            color: get_color(CharColor::DEFAULT),
        });
    }
}

pub fn update_text(
    mut screen_txt: ResMut<ScreenText>,
    mut input: EventReader<ReceivedCharacter>,
    typed: Res<Input<KeyCode>>,
    mut current_char_index: ResMut<CurrentCharIndex>,
) {
    let current_char = screen_txt.0.get_mut(current_char_index.0).unwrap();
    for c in input.iter() {
        if c.char == current_char.text {
            current_char.color = get_color(CharColor::CORRECT);
            current_char_index.0 += 1;
        } else if typed.just_pressed(KeyCode::Back) {
            current_char.color = get_color(CharColor::DEFAULT);
            current_char_index.0 -= 1;
        } else {
            current_char.color = get_color(CharColor::WRONG);
            current_char_index.0 += 1; // REVIEW: do you still want to increment if the character is wrong?
        }
    }
}


pub fn draw_text(
    mut commands: Commands,
    screen_txt: Res<ScreenText>,
    mut query: Query<(Entity, &Text)>,
) {

    for (entity, _) in query.iter_mut() {
        commands.entity(entity).despawn();
    }
    
    commands.spawn(TextBundle::from_sections(
        screen_txt.0.iter().map(|c| {
            TextSection {
                value: c.text.to_string(),
                style: TextStyle {
                    font_size: FONT_SIZE,
                    color: c.color,
                    ..Default::default()
                },
            }
        }).collect::<Vec<_>>()
    ));

}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 100.0),
        ..default()
    });
}
