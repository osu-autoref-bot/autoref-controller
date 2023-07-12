use async_trait::async_trait;
use osu_match_lib::multi::OsuMultiRoom;

use crate::error::AutoRefError;

use super::BaseMatch;

#[derive(Debug)]
struct Qualify {
    room: OsuMultiRoom,
}

#[async_trait]
impl BaseMatch for Qualify {
    type Constructor = Self;
    async fn new(room_name: &str) -> Result<Self::Constructor, AutoRefError> {
        Ok(Self {
            room: OsuMultiRoom::new(room_name).await?,
        })
    }

    async fn close(&self) -> Result<(), AutoRefError> {
        self.room.close().await.map_err(AutoRefError::MatchLibError)
    }
    async fn change_map(&mut self, map_id: u64) -> Result<(), AutoRefError> {    
        self.room.set_map(map_id).await.map_err(AutoRefError::MatchLibError)
    }
    
    async fn start(&self)  -> Result<(), AutoRefError> {
        self.room.start(None).await.map_err(AutoRefError::MatchLibError)
    }
}
