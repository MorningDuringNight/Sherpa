use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use crate::player::Player;


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


pub fn updateHeight(
    mut maxheight: ResMut<MaxHeight>,
    players: Query<&Transform, With<Player>>
){
    for player in players.iter(){
        if player.translation.y as u32 > maxheight.amount{
            maxheight.amount = player.translation.y as u32;
        }
    }
}

pub fn loadUI(
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

pub fn updateUI(
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
