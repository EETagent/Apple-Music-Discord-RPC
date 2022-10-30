#![allow(non_camel_case_types)]

use std::{fmt};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum iTunesAppName {
    Music,
    iTunes,
}

// To string
impl fmt::Display for iTunesAppName {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct iTunesInfos {
    pub artwork: Option<String>,
    pub url: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct iTunesProps {
    pub class: String,
    pub id: i64,
    pub index: i64,
    pub name: String,
    #[serde(rename = "persistentID")]
    pub persistent_id: String,
    #[serde(rename = "databaseID")]
    pub database_id: i64,
    #[serde(skip_deserializing)]
    pub date_added: String,
    pub time: String,
    pub duration: Option<f64>,
    pub artist: String,
    pub album_artist: String,
    pub composer: String,
    pub album: String,
    pub genre: String,
    pub bit_rate: i64,
    pub sample_rate: i64,
    pub track_count: i64,
    pub track_number: i64,
    pub disc_count: i64,
    pub disc_number: i64,
    pub size: i64,
    pub volume_adjustment: i64,
    pub year: i64,
    pub comment: String,
    pub eq: String,
    pub kind: String,
    pub media_kind: String,
    #[serde(skip_deserializing)]
    pub modification_date: String,
    pub enabled: bool,
    pub start: i64,
    pub finish: f64,
    pub played_count: i64,
    pub skipped_count: i64,
    pub compilation: bool,
    pub rating: i64,
    pub bpm: i64,
    pub grouping: String,
    pub bookmarkable: bool,
    pub bookmark: i64,
    pub shufflable: bool,
    pub lyrics: String,
    pub category: String,
    pub description: String,
    pub episode_number: i64,
    pub unplayed: bool,
    pub sort_name: String,
    pub sort_album: String,
    pub sort_artist: String,
    pub sort_composer: String,
    pub sort_album_artist: String,
    #[serde(skip_deserializing)]
    pub release_date: String,
    pub loved: bool,
    pub disliked: bool,
    pub album_loved: bool,
    pub album_disliked: bool,
    pub cloud_status: String,
    pub work: String,
    pub movement: String,
    pub movement_number: i64,
    pub movement_count: i64,
    #[serde(skip_deserializing)]
    pub location: String,
}
