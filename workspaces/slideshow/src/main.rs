use bevy::{prelude::*, window::PresentMode};

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

#[derive(Component)]
struct Despawnable;

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
        //.add_systems(Startup, despawn)
        .add_systems(Update, show_popup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite::from_image(asset_server.load("sprites/Picture720-Ket-Hwa.png")));
    //despawn(commands, asset_server,);
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/AAAaaa.png")),
        Transform {
            translation: Vec3::new(0., 0., -2.),
            ..default()
        },
        PopupTimer(Timer::from_seconds(2. + 0.1, TimerMode::Once)), Despawnable,
        //DespawnTimer(Timer::from_seconds(0.1, TimerMode::Once))
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/safeimagekit-ZhuoyanCen.png")),
        Transform {
            translation: Vec3::new(0., 0., -1.2),
            ..default()
        },
        PopupTimer(Timer::from_seconds(4.+ 0.1, TimerMode::Once)), Despawnable,
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/JaggerSlideshowImg.png")),
        Transform {
            translation: Vec3::new(0., 0., -1.4),
            ..default()
        },
        PopupTimer(Timer::from_seconds(6.+ 0.1, TimerMode::Once)), Despawnable,
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/shmulPixel.png")),
        Transform {
            translation: Vec3::new(0., 0., -1.6),
            ..default()
        },
        PopupTimer(Timer::from_seconds(8.+ 0.1, TimerMode::Once)), Despawnable,
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/7952590F.png")),
        Transform {
            translation: Vec3::new(0., 0., -1.8),
            ..default()
        },
        PopupTimer(Timer::from_seconds(10.+ 0.1, TimerMode::Once)),Despawnable,
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/alli_pixel.png")),
        Transform {
            translation: Vec3::new(0., 0., -2.),
            ..default()
        },
        PopupTimer(Timer::from_seconds(12.+ 0.1, TimerMode::Once)),Despawnable,
    ));
    //info!("Hello world!");
}

fn show_popup(mut commands: Commands, time: Res<Time>, mut popup: Query<(Entity, &mut PopupTimer, &mut Transform), With <Despawnable>>) {
    //let mut checker = 0;
    for (entity, mut timer, mut transform) in popup.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z = 1.;
            info!("Should be Linux!");
            //sleep
            //commands.entity(entity).despawn();
            timer.reset();
        }
        //info!("Kill me XD");
        //commands.entity(entity).despawn();
    }
    
}

fn despawn(mut commands: Commands, cute: Query<Entity, With<Despawnable>>){
    for entity in cute.iter(){
        info!("despawn trigger");
        commands.entity(entity).despawn();
    }

}