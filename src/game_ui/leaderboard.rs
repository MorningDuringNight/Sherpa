use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufWriter,BufReader, BufRead, Write};
use std::cmp::Reverse;

use crate::app::GameMode;
use crate::game_ui::{MaxHeight, TotalCoin};

pub struct LeaderboardEntry{
    pub gametype: String,
    pub coin: u32,
    pub score: u32,
}



pub fn read_leaderboard(
) -> Vec<LeaderboardEntry> {
    let file = File::open("src/game_ui/leaderboard.txt").expect("unable to open");
    let reader = BufReader::new(file);
    let mut entries = Vec::new();
    let mut lines = reader.lines();
    for line in lines{
        let entry = line.expect("unable");
        let parts: Vec<&str>= entry.split_whitespace().collect();
        let coin = parts[1].parse::<u32>().unwrap_or(0);
        let score = parts[2].parse::<u32>().unwrap_or(0);

        entries.push(LeaderboardEntry{
        gametype: parts[0].to_string(),
        coin,
        score
        });
    }
    return entries;

}

pub fn update_leaderboard(
    coinCount: Res<TotalCoin>,
    maxScore: Res<MaxHeight>,
    gameType: Res<GameMode>,
){
    let mut entries = read_leaderboard();

    let typestring = 
    
    match *gameType {
        GameMode::LocalCoop => {
            "Player"
        }
        GameMode::LocalWithNpc(local_player_number) => {
            "AI"
        }
        GameMode::AiWithAi => {
            "AI"
        }
        GameMode::NetCoop(local_player_number) => {
            "Player"
        }
        GameMode::Simulated => {
            "Simulated"
        }
    };

    entries.push(LeaderboardEntry{
        gametype: typestring.to_string(),
        coin: coinCount.amount,
        score: maxScore.amount,
    });
    entries.sort_by_key(|LeaderboardEntry| Reverse(LeaderboardEntry.score));

    let file = OpenOptions::new().write(true).open("src/game_ui/leaderboard.txt").expect("unable to open");
    let mut writer = BufWriter::new(file);

    let mut count = 0;
    for line in entries {
        writeln!(writer, "{} {} {}", line.gametype, line.coin.to_string(), line.score.to_string()).expect("unable to write");
        count += 1;
        if count == 10{
            break;
        }
    }
    writer.flush().expect("unable to write");
}