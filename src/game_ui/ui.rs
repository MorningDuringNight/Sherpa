use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use crate::player::Player;
use crate::components::motion::Position;
use crate::map::{Collider, MapFile, MapTextureHandles, MapDimensions, AtlasLayoutResource, game_objects, ground};


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
            .add_systems(Update, (render_gameover_ui, handle_gameover_input, reload_map_on_restart))
            .add_systems(Update, updateUI);
    }
}

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
            
            Text::new("0"), 
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
            
            Text::new("0"), 
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
    mut players: Query<(&mut Transform, &mut crate::components::motion::Velocity, &mut crate::components::motion::Momentum, &mut crate::components::motion::Position), With<Player>>,
){
    if !game_over.active { return; }

        // 重开：R
        if keyboard.just_pressed(KeyCode::KeyR) {
            println!("重新开始游戏...");
            
            // 先重置分数和硬币计数
            coinCount.amount = 0;
            maxScore.amount = 0;
            
            // 重置到初始生成位置
            let mut player_count = 0;
            for (mut trans, mut vel, mut mom, mut pos) in &mut players {
                if player_count == 0 {
                    // 第一个玩家：初始位置 (50, 0, 0)
                    trans.translation.x = 50.0;
                    trans.translation.y = 0.0;
                    pos.0.x = 50.0;
                    pos.0.y = 0.0;
                } else {
                    // 第二个玩家：初始位置 (350, 0, 0)
                    trans.translation.x = 350.0;
                    trans.translation.y = 0.0;
                    pos.0.x = 350.0;
                    pos.0.y = 0.0;
                }
                vel.0 = Vec2::ZERO;
                mom.0 = Vec2::ZERO;
                player_count += 1;
            }
            
            // 最后设置游戏状态为进行中
            game_over.active = false;
        }

    // 退出：Esc
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(bevy::app::AppExit::Success);
    }
}

// 重新加载地图的系统
pub fn reload_map_on_restart(
    mut commands: Commands,
    map: Res<MapFile>,
    images: Res<MapTextureHandles>,
    atlas: Res<AtlasLayoutResource>,
    map_dimensions: Res<MapDimensions>,
    game_over: Res<GameOver>,
    keyboard: Res<ButtonInput<KeyCode>>,
    // 查询所有地图实体（除了玩家）
    map_entities: Query<Entity, (With<Collider>, Without<Player>)>,
) {
    // 只在游戏结束且按R键时重新加载地图
    if game_over.active && keyboard.just_pressed(KeyCode::KeyR) {
        
        // 清理所有地图实体（除了玩家）
        for entity in map_entities.iter() {
            commands.entity(entity).despawn();
        }
        
        // 重新生成地图实体
        let map_entities = game_objects(&images.entity, &atlas, &map, map_dimensions.h);
        for game_entity in map_entities {
            game_entity.spawn(&mut commands);
        }
        
        // 重新生成地面
        let ground_entity = ground();
        ground_entity.spawn(&mut commands);
    }
}
