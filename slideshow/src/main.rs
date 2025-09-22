use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SlideTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
        .insert_resource(SlideIndex(0))
        .add_systems(Startup, setup)
        .add_systems(Update, change_color)
        .run();
}

#[derive(Resource)]
struct SlideTimer(Timer);

#[derive(Resource)]
struct SlideIndex(usize);

#[derive(Component)]
struct Slide;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Spawn sprite manually without SpriteBundle
    commands.spawn((
        Sprite {
            color: COLORS[0],
            custom_size: Some(Vec2::new(200.0, 200.0)),
            ..default()
        },
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        Slide,
    ));
}

fn change_color(
    time: Res<Time>,
    mut timer: ResMut<SlideTimer>,
    mut index: ResMut<SlideIndex>,
    mut query: Query<&mut Sprite, With<Slide>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        index.0 = (index.0 + 1) % COLORS.len();
        if let Ok(mut sprite) = query.get_single_mut() {
            sprite.color = COLORS[index.0];
        }
    }
}

const COLORS: [Color; 5] = [
    Color::srgb(1.0, 0.0, 0.0),
    Color::srgb(0.0, 1.0, 0.0),
    Color::srgb(0.0, 0.0, 1.0),
    Color::srgb(1.0, 1.0, 0.0),
    Color::srgb(0.5, 0.0, 0.5),
];
