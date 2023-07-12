use async_trait::async_trait;
use osu_match_lib::multi::OsuMultiRoom;

use crate::error::AutoRefError;

use super::BaseMatch;

#[derive(Debug)]
struct TeamMatch {
    room: OsuMultiRoom,
}

#[async_trait]
impl BaseMatch for TeamMatch {
    type Constructor = Self;
    async fn new(room_name: &str) -> Result<Self::Constructor, AutoRefError> {
        Ok(Self {
            room: OsuMultiRoom::new(room_name).await?,
        })
    }

    async fn close(&self) -> Result<(), AutoRefError> {
        self.room.close().await.map_err(AutoRefError::MatchLibError)
    }
}
