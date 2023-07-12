use std::collections::HashMap;

use async_trait::async_trait;
use osu_match_lib::multi::{Mods, ScoreSetting, TeamSetting};

use crate::error::AutoRefError;

pub mod qualify;
// pub mod team_matches;

#[async_trait]
trait BaseMatch {
    type Constructor;
    async fn new(room_name: &str) -> Result<Self::Constructor, AutoRefError>;

    async fn close(&self) -> Result<(), AutoRefError>;

    async fn change_map(&mut self, map_id: u64) -> Result<(), AutoRefError>;

    async fn start(&self)  -> Result<(), AutoRefError>;
}

#[derive(Debug, Clone, Copy)]
pub enum OsuMatchStatus {
    Ban(Team),
    Protect(Team),
    Pick(Team),
    PostPick,
    Playing,
}

#[derive(Debug, Clone, Copy)]
pub enum Team {
    Red,
    Blue,
}

#[derive(Debug, Clone)]
pub struct TeamDetails {
    name: String,
    score: u8,
    lineup: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MatchMap {
    map_id: u64,
    mods: Vec<Mods>,
    team_settings: TeamSetting,
    score_settings: ScoreSetting,
}

type MapPool = HashMap<(String, u8), MatchMap>;
