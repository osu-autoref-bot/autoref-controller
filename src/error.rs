use std::num::ParseIntError;

use osu_match_lib::error::IrcMatchError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AutoRefError {
    #[error(transparent)]
    MatchLibError(#[from] IrcMatchError),
    #[error(transparent)]
    Unclassified(#[from] anyhow::Error),
}
