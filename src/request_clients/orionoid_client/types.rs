use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionApiResponse {
    pub name: Option<String>,
    pub version: Option<String>,
    pub result: Option<OrionResult>,
    pub data: Option<OrionData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionResult {
    pub status: Option<String>,
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    pub description: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionData {
    #[serde(rename = "type")]
    pub data_type: Option<String>,
    pub movie: Option<OrionMovie>,
    pub show: Option<OrionShow>,
    pub episode: Option<OrionEpisode>,
    pub count: Option<OrionCount>,
    pub streams: Option<Vec<OrionStream>>,
    pub requests: Option<OrionRequests>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionMovie {
    pub id: Option<OrionMovieId>,
    pub time: Option<OrionTime>,
    pub meta: Option<OrionMeta>,
    pub popularity: Option<OrionPopularity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionMovieId {
    pub orion: Option<String>,
    pub imdb: Option<String>,
    pub tmdb: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionShow {
    #[serde(rename = "type")]
    pub show_type: Option<String>,
    pub id: Option<OrionShowId>,
    pub time: Option<OrionTime>,
    pub popularity: Option<OrionPopularity>,
    pub meta: Option<OrionMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionShowId {
    pub orion: Option<String>,
    pub imdb: Option<String>,
    pub tmdb: Option<String>,
    pub tvdb: Option<String>,
    pub tvrage: Option<String>,
    pub trakt: Option<String>,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionEpisode {
    #[serde(rename = "type")]
    pub episode_type: Option<String>,
    pub id: Option<OrionEpisodeId>,
    pub time: Option<OrionTime>,
    pub popularity: Option<OrionPopularity>,
    pub number: Option<OrionEpisodeNumber>,
    pub meta: Option<OrionMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionEpisodeId {
    pub orion: Option<String>,
    pub imdb: Option<String>,
    pub tmdb: Option<String>,
    pub tvdb: Option<String>,
    pub tvrage: Option<String>,
    pub trakt: Option<String>,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionEpisodeNumber {
    pub season: Option<u32>,
    pub episode: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionTime {
    pub added: Option<u64>,
    pub updated: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionMeta {
    pub title: Option<String>,
    pub year: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionPopularity {
    pub count: Option<u32>,
    pub percent: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionCount {
    pub total: Option<u32>,
    pub requested: Option<u32>,
    pub retrieved: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionStream {
    pub id: Option<String>,
    pub time: Option<OrionTime>,
    pub links: Option<Vec<String>>,
    pub stream: Option<OrionStreamInfo>,
    pub access: Option<OrionAccess>,
    pub file: Option<OrionFile>,
    pub meta: Option<OrionStreamMeta>,
    pub video: Option<OrionVideo>,
    pub audio: Option<OrionAudio>,
    pub subtitle: Option<OrionSubtitle>,
    pub popularity: Option<OrionPopularity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionStreamInfo {
    #[serde(rename = "type")]
    pub stream_type: Option<String>,
    pub source: Option<String>,
    pub hoster: Option<String>,
    pub seeds: Option<u32>,
    pub time: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionAccess {
    pub direct: Option<bool>,
    pub premiumize: Option<bool>,
    pub offcloud: Option<bool>,
    pub torbox: Option<bool>,
    pub easydebrid: Option<bool>,
    pub realdebrid: Option<bool>,
    pub alldebrid: Option<bool>,
    pub debridlink: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionFile {
    pub hash: Option<String>,
    pub name: Option<String>,
    pub size: Option<u64>,
    pub pack: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionStreamMeta {
    pub release: Option<String>,
    pub uploader: Option<String>,
    pub edition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionVideo {
    pub quality: Option<String>,
    pub codec: Option<String>,
    #[serde(rename = "3d")]
    pub is_3d: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionAudio {
    #[serde(rename = "type")]
    pub audio_type: Option<String>,
    pub channels: Option<u8>,
    pub system: Option<String>,
    pub codec: Option<String>,
    pub languages: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionSubtitle {
    #[serde(rename = "type")]
    pub subtitle_type: Option<String>,
    pub languages: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionRequests {
    pub total: Option<u32>,
    pub daily: Option<OrionDailyRequests>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrionDailyRequests {
    pub limit: Option<u32>,
    pub used: Option<u32>,
    pub remaining: Option<u32>,
}