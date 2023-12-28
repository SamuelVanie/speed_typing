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

fn char_width(c: char, font_size: f32) -> f32 {
    // May need to adjust this based on your font and size
    // This is a rough estimate assuming a monospaced font
    // May need a more accurate measurement based on your actual font
    (c as u32).count_ones() as f32 * font_size
}

pub fn draw_text(
    mut commands: Commands,
    screen_txt: Res<ScreenText>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let mut x = 0.0;
    let mut y = 0.0;
    let margin = 10.0;
    let font_size = (50.0, 50.0);
    let window = window_query.get_single().unwrap();

    for c in screen_txt.0.iter() {
        let char_width = char_width(c.text, font_size.0);
        if x + char_width <= (window.width() - margin) {
            commands.spawn((TextBundle::from_section(
                c.text.to_string(),
                TextStyle {
                    font_size: font_size.0,
                    color: c.color,
                    ..Default::default()
                },
            )
            .with_text_alignment(TextAlignment::Left)
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(y + 5.0),
                right: Val::Px(window.width() - x - char_width - margin), // Adjust position based on available space
                ..Default::default()
            }),));

            x += char_width;
        } else {
            // Move to the next line
            x = 0.0;
            y += font_size.0;
        }
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 100.0),
        ..default()
    });
}
