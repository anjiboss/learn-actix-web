// define what we expecting in a body for a post
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct CreateEntryData {
    pub title: String,
    pub date: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdateEntryData {
    pub title: String,
}
