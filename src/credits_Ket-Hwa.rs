use bevy::{prelude::*, window::PresentMode};

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "credits!".into(),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, show_popup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite::from_image(asset_server.load("sprites/WIN_20250920_13_21_03_Pro.png")));
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/testchar.png")),
        Transform {
            translation: Vec3::new(0., 0., -1.),
            ..default()
        },
        PopupTimer(Timer::from_seconds(2., TimerMode::Once)),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/testchar.png")),
        Transform {
            translation: Vec3::new(0., 0., -2.),
            ..default()
        },
        PopupTimer(Timer::from_seconds(4., TimerMode::Once)),
    ));

    info!("Hello world!");
}

fn show_popup(time: Res<Time>, mut popup: Query<(&mut PopupTimer, &mut Transform)>) {
    for (mut timer, mut transform) in popup.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z = 2.;
            info!("Should be Linux!");
        }
    }
}