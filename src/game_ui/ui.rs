use crate::app::{Background, GameAssets, GameMode, ToggleBotEvent};
use crate::config::MyAppState;
use crate::physics::MaxHeightReached;
use crate::player::Player;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

use bevy::color::palettes::css::BLACK;
use bevy::color::palettes::css::BLUE;

use crate::game_ui::read_leaderboard;

#[derive(Component)]
pub struct UICamera;

#[derive(Resource, Debug, Default, Clone, PartialEq, Eq)]
pub struct TotalCoin {
    pub amount: u32,
}

#[derive(Resource, Debug, Default, Clone, PartialEq, Eq)]
pub struct MaxHeight {
    pub amount: u32,
}

#[derive(Component)]
pub struct CoinDisplay;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct WinDisplay;

#[derive(Component)]
pub struct EntryDisplay;

// impl Plugin for UIPlugin{
//     fn build(&self, app: &mut App){
//         app
//             .insert_resource( TotalCoin {amount:0,})
//             .insert_resource(MaxHeight{amount:0,})
//             .add_systems(Startup, load_ui_camera)
//             .add_systems(OnEnter(MyAppState::InGame), load_ui_game)
//             .add_systems(OnEnter(MyAppState::MainMenu), load_main_menu)
//             .add_systems(Update, main_menu_input
//                 .run_if(in_state(MyAppState::MainMenu)))
//             .add_systems(Update, update_height
//                 .run_if(in_state(MyAppState::InGame)))
//             .add_systems(Update, update_ui
//                 .run_if(in_state(MyAppState::InGame)))
//             .add_systems(OnExit(MyAppState::MainMenu), despawn_ui);

//     }
// }

pub fn update_height(mut maxheight: ResMut<MaxHeight>, players: Query<&Transform, With<Player>>) {
    for player in players.iter() {
        if player.translation.y as u32 > maxheight.amount {
            maxheight.amount = player.translation.y as u32;
        }
    }
}

pub fn load_ui_game(mut commands: Commands) {
    commands
        .spawn((Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            margin: UiRect::all(Val::Percent(0.)),
            padding: UiRect::all(Val::Percent(0.)),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Percent(2.),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(5.),
                    ..Default::default()
                },
                (Text::new("Coins: "), TextColor(BLACK.into())),
                RenderLayers::layer(1),
            ));
            parent.spawn((
                Node {
                    width: Val::Percent(10.),
                    ..Default::default()
                },
                (Text::new("coins"), TextColor(BLACK.into())),
                RenderLayers::layer(1),
                CoinDisplay,
            ));
            parent.spawn((
                Node {
                    width: Val::Percent(5.),
                    ..Default::default()
                },
                (Text::new("Score: "), TextColor(BLACK.into())),
                RenderLayers::layer(1),
            ));
            parent.spawn((
                Node {
                    width: Val::Percent(10.),
                    ..Default::default()
                },
                (Text::new("score"), TextColor(BLACK.into())),
                RenderLayers::layer(1),
                ScoreDisplay,
            ));
        });
}

pub fn update_ui(
    coinCount: Res<TotalCoin>,
    maxScore: Res<MaxHeight>,
    mut query_coin: Query<&mut Text, With<CoinDisplay>>,
    mut query_score: Query<&mut Text, (With<ScoreDisplay>, Without<CoinDisplay>)>,
) {
    for mut text in query_coin.iter_mut() {
        text.0 = coinCount.amount.to_string();
    }

    for mut text in query_score.iter_mut() {
        text.0 = maxScore.amount.to_string();
    }
}

pub fn load_ui_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 1, // draw after player camera
            clear_color: ClearColorConfig::None,
            ..default()
        },
        RenderLayers::layer(1),
        UICamera,
    ));
}
//home page stuff

pub fn load_main_menu(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_image(game_assets.main_menu.clone()),
        Transform::from_xyz(0., 0., -1.),
        RenderLayers::layer(1),
        Background,
    ));
}

pub fn main_menu_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MyAppState>>,
    mut coin_count: ResMut<TotalCoin>,
    mut height: ResMut<MaxHeight>,
    #[cfg(feature = "client")] mut ev_toggle: EventWriter<ToggleBotEvent>,
) {
    let mode = if keyboard_input.just_pressed(KeyCode::Digit1) {
        info!("Pressed 1 → Starting game as net coop P1");
        Some(GameMode::NetCoop(0))
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        info!("Pressed 2 → Starting game as net coop P2");
        Some(GameMode::NetCoop(1))
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        info!("Pressed 3 → Starting game with npc");
        #[cfg(feature = "client")]
        ev_toggle.write(ToggleBotEvent);
        Some(GameMode::LocalWithNpc(0))
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        info!("Pressed 4 → Starting game bot+bot");
        // also send toggle event
        #[cfg(feature = "client")]
        ev_toggle.write(ToggleBotEvent);
        Some(GameMode::AiWithAi)
    } else if keyboard_input.just_pressed(KeyCode::KeyS) {
        info!("Pressed S → Starting game local coop");
        Some(GameMode::LocalCoop)
    } else {
        None
    };

    if let Some(mode) = mode {
        commands.insert_resource(mode);
        coin_count.amount = 0;
        height.amount = 0;

        next_state.set(MyAppState::InGame);
    }
}

pub fn game_death(
    mut ev_r: EventReader<MaxHeightReached>,
    mut next_state: ResMut<NextState<MyAppState>>,
) {
    for ev in ev_r.read() {
        next_state.set(MyAppState::EndCredit);
    }
}

///fix
pub fn despawn_ui(
    mut commands: Commands,
    mut background: Query<Entity, With<Background>>,
    mut objects: Query<Entity, (With<StateScoped<MyAppState>>)>,
    mut nodes: Query<Entity, With<Node>>,
) {
    for object in background.iter_mut() {
        commands.entity(object).despawn();
    }
    for object in objects.iter_mut() {
        commands.entity(object).despawn();
    }
    for object in nodes.iter_mut() {
        commands.entity(object).despawn();
    }
}

//leaderboard_ui

pub fn load_ui_leaderboard(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn((Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            margin: UiRect::all(Val::Percent(0.)),
            padding: UiRect::all(Val::Percent(0.)),
            flex_direction: FlexDirection::Column,
            column_gap: Val::Percent(2.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },))
        .with_children(|header| {
            header.spawn((
                Node {
                    width: Val::Percent(50.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0.7, 0.8, 0.9)),
                (Text::new("You Lose!"), TextColor(BLACK.into())),
                RenderLayers::layer(1),
                WinDisplay,
            ));
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(50.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                (
                    Text::new("Type     Coins     Score"),
                    TextColor(BLACK.into()),
                ),
                BackgroundColor(Color::srgb(0.7, 0.8, 0.9)),
                RenderLayers::layer(1),
            ));
            for i in 0..10 {
                parent.spawn((
                    Node {
                        width: Val::Percent(50.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    (Text::new("Unknown Value"), TextColor(BLACK.into())),
                    BackgroundColor(Color::srgb(0.7, 0.8, 0.9)),
                    RenderLayers::layer(1),
                    EntryDisplay,
                ));
            }
        });

    commands.spawn((
        Sprite::from_image(game_assets.background.clone()),
        Transform::from_xyz(0., 0., 1.),
        RenderLayers::layer(1),
        Background,
    ));
}

pub fn update_end(
    maxScore: Res<MaxHeight>,
    mut query_entry: Query<&mut Text, With<EntryDisplay>>,
    mut query_win: Query<&mut Text, (With<WinDisplay>, Without<EntryDisplay>)>,
) {
    if maxScore.amount >= 2000 {
        for mut win in query_win.iter_mut() {
            win.0 = "You Win!!!".to_string();
        }
    }
    let entities = read_leaderboard();
    println!("reading");
    for (mut line, entity) in query_entry.into_iter().zip(entities) {
        println!("def reading");
        line.0 = (entity.gametype
            + "     "
            + &entity.coin.to_string()
            + "     "
            + &entity.score.to_string());
    }
}
