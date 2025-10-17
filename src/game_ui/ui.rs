use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use crate::player::Player;
use crate::components::motion::Position;

pub struct UIPlugin;

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

#[derive(Resource, Debug, Default, Clone, PartialEq, Eq)]
pub struct GameOver{
    pub active: bool,
}

#[derive(Component)]
pub struct GameOverRoot;

impl Plugin for UIPlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource( TotalCoin {amount:0,})
            .insert_resource(MaxHeight{amount:0,})
            .insert_resource(GameOver{active:false})
            .add_systems(Startup, loadUI)
            .add_systems(Update, updateHeight)
            .add_systems(Update, updateUI)
            .add_systems(Update, (render_gameover_ui, handle_gameover_input));
    }
}

pub fn updateHeight(
    mut maxheight: ResMut<MaxHeight>,
    players: Query<&Position, With<Player>>
){
    for player in players.iter(){
        if player.0.y as u32 > maxheight.amount{
            maxheight.amount = player.0.y as u32;
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

pub fn render_gameover_ui(
    mut commands: Commands,
    game_over: Res<GameOver>,
    existing: Query<Entity, With<GameOverRoot>>,
    coinCount: Res<TotalCoin>,
    maxScore: Res<MaxHeight>,
){
    let exists = existing.get_single().ok();
    if game_over.active {
        if exists.is_none() {
            // 覆盖层
            let root = commands.spawn((
                Node{
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                RenderLayers::layer(1),
                GameOverRoot,
            )).id();
            commands.entity(root).with_children(|parent|{
                parent.spawn((
                    Node{
                        width: Val::Px(520.),
                        height: Val::Px(260.),
                        padding: UiRect::all(Val::Px(24.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    RenderLayers::layer(1),
                ))
                .with_children(|panel|{
                    panel.spawn((
                        Text::new(format!(
                            "Game Over\nCoins: {}\nScore: {}\nPress R to Restart, Esc to Exit",
                            coinCount.amount,
                            maxScore.amount,
                        )),
                        RenderLayers::layer(1),
                    ));
                });
            });
        }
    } else {
        if let Some(e) = exists { commands.entity(e).despawn_recursive(); }
    }
}

pub fn handle_gameover_input(
    mut game_over: ResMut<GameOver>,
    mut coinCount: ResMut<TotalCoin>,
    mut maxScore: ResMut<MaxHeight>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<bevy::app::AppExit>,
    mut players: Query<(&mut Transform, &mut crate::components::motion::Velocity, &mut crate::components::motion::Momentum), With<Player>>,
){
    if !game_over.active { return; }

    // 重开：R
    if keyboard.just_pressed(KeyCode::KeyR) {
        game_over.active = false;
        coinCount.amount = 0;
        maxScore.amount = 0;
        // 重置到最底部，并清零速度与动量，避免“从上落下”的效果
        for (mut trans, mut vel, mut mom) in &mut players {
            trans.translation.y = 0.0;
            vel.0 = Vec2::ZERO;
            mom.0 = Vec2::ZERO;
        }
    }

    // 退出：Esc
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(bevy::app::AppExit::Success);
    }
}
