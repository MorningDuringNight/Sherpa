use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use crate::player::Player;
use crate::config::MyAppState;
use crate::app::{Background, GameAssets};


#[derive(Component)]
pub struct UICamera;

#[derive(Resource, Debug, Default, Clone, PartialEq, Eq)]
pub struct TotalCoin{
    pub amount: u32,
}


#[derive(Resource, Debug, Default, Clone, PartialEq, Eq)]
pub struct MaxHeight{
    pub amount: u32,
}

#[derive(Component)]
pub struct CoinDisplay;


#[derive(Component)]
pub struct ScoreDisplay;

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

pub fn update_height(
    mut maxheight: ResMut<MaxHeight>,
    players: Query<&Transform, With<Player>>
){
    for player in players.iter(){
        if player.translation.y as u32 > maxheight.amount{
            maxheight.amount = player.translation.y as u32;
        }
    }
}

pub fn load_ui_game(
    mut commands: Commands, 
){
    commands.spawn((
        Node{
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            margin: UiRect::all(Val::Percent(0.)),
            padding: UiRect::all(Val::Percent(0.)),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Percent(2.),
            ..default()
        },  
    ))
    .with_children(|parent|{
        parent.spawn((
                Node {
                    width: Val::Percent(5.),
                    ..Default::default()
                },
            
            Text::new("Coins: "), 
            RenderLayers::layer(1),
        ));
        parent.spawn((
                Node {
                    width: Val::Percent(10.),
                    ..Default::default()
                },
            
            Text::new("coins"), 
            RenderLayers::layer(1),
            CoinDisplay,
        ));
        parent.spawn((
                Node {
                    width: Val::Percent(5.),
                    ..Default::default()
                },
            
            Text::new("Score: "), 
            RenderLayers::layer(1),
        ));
        parent.spawn((
                Node {
                    width: Val::Percent(10.),
                    ..Default::default()
                },
            
            Text::new("score"), 
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
){
   
    for mut text in query_coin.iter_mut(){
        text.0 = coinCount.amount.to_string();
    }

    for mut text in query_score.iter_mut(){
        text.0 = maxScore.amount.to_string();
    }
}

pub fn load_ui_camera(
    mut commands: Commands,
){
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

pub fn load_main_menu(
    mut commands: Commands, 
    game_assets: Res<GameAssets>,
){
    commands.spawn((
        Sprite::from_image(game_assets.main_menu.clone()),
        Transform::from_xyz(0., 0., -1.),
        RenderLayers::layer(1),
        Background,
    ));
}

pub fn main_menu_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MyAppState>>
){
    if keyboard_input.pressed(KeyCode::KeyM){

    }
    if keyboard_input.pressed(KeyCode::KeyS){
        next_state.set(MyAppState::InGame);
    }

}

pub fn despawn_ui(
    mut commands: Commands,
    mut background: Query<Entity, With<Background>>,
){
    for bg in background.iter_mut(){
        commands.entity(bg).despawn();
    }
}
