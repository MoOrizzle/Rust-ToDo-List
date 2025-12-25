use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListEntry {
    pub time_stamp: DateTime<Local>,
    pub title: String,
    pub text: String,
}