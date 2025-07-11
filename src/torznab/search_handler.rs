use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::OnceCell;
use crate::request_clients::orionoid_client::orionoid_request_client::{OrionoidRequestClient};
use crate::request_clients::orionoid_client::types::*;
use crate::torznab;
use crate::torznab::categories::get_category_by_name;
use crate::torznab::types::Torrent;

static ORIONOID_API_CLIENT: OnceCell<Arc<OrionoidRequestClient>> = OnceCell::new();

fn get_orionoid_client() -> Arc<OrionoidRequestClient> {
    ORIONOID_API_CLIENT.get().expect("Orionoid API Client not initialized").clone()
}

enum OrionIdRef<'a> {
    Movie(&'a OrionMovieId),
    Episode(&'a OrionEpisodeId),
    Show(&'a OrionShowId),
}

fn map_orion_api_response_to_torrents(
    api_response: OrionApiResponse,
) -> Vec<Torrent> {
    let data = match api_response.data {
        Some(data) => data,
        None => return Vec::new(),
    };

    let show_title = data.show.as_ref()
        .and_then(|show| show.meta.as_ref())
        .and_then(|meta| meta.title.as_ref());

    let episode_title = data.episode.as_ref()
        .and_then(|ep| ep.meta.as_ref())
        .and_then(|meta| meta.title.as_ref());

    let (meta_main, id, season_episode, is_episode) = if let Some(movie) = &data.movie {
        (movie.meta.as_ref(), movie.id.as_ref().map(OrionIdRef::Movie), None, false)
    } else if let Some(episode) = &data.episode {
        let se = episode.number.as_ref().map(|n| (n.season, n.episode));
        (episode.meta.as_ref(), episode.id.as_ref().map(OrionIdRef::Episode), se, true)
    } else if let Some(show) = &data.show {
        (show.meta.as_ref(), show.id.as_ref().map(OrionIdRef::Show), None, false)
    } else {
        (None, None, None, false)
    };

    let streams = match data.streams {
        Some(streams) => streams,
        None => return Vec::new(),
    };

    let category_ids = if data.movie.is_some() {
        get_category_by_name("Movies").map(|c| vec![c.id]).unwrap_or_default()
    } else if data.show.is_some() || data.episode.is_some() {
        get_category_by_name("TV").map(|c| vec![c.id]).unwrap_or_default()
    } else {
        Vec::new()
    };

    streams.into_iter().filter_map(|stream| {
        let meta = stream.meta.as_ref();
        let file = stream.file.as_ref();
        let stream_info = stream.stream.as_ref();
        let video = stream.video.as_ref();
        let audio = stream.audio.as_ref();
        let links = stream.links.as_ref();

        let size = file.and_then(|f| f.size);

        let title = if is_episode {
            if let (Some(show), Some(ep), Some((Some(season), Some(episode)))) = (show_title, episode_title, season_episode) {
                Some(format!("{} - {} - S{:02}E{:02}", show, ep, season, episode))
            } else {
                // fallback to episode or show title
                episode_title.cloned().or_else(|| show_title.cloned())
            }
        } else {
            meta_main.and_then(|m| m.title.as_ref()).cloned()
        };

        if size.is_none() || title.is_none() || links.is_none() {
            return None;
        }

        let mut other_attributes = HashMap::new();
        let mut result_type = "movie";
        if let Some(id) = &id {
            match id {
                OrionIdRef::Movie(id) => {
                    if let Some(orion) = &id.orion {
                        other_attributes.insert("orion_id".to_string(), orion.clone());
                    }
                    if let Some(imdb) = &id.imdb {
                        other_attributes.insert("imdb".to_string(), imdb.clone());
                    }
                    if let Some(tmdb) = &id.tmdb {
                        other_attributes.insert("tmdbid".to_string(), tmdb.clone());
                    }
                }
                OrionIdRef::Episode(id) => {
                    if let Some(orion) = &id.orion {
                        other_attributes.insert("orion_id".to_string(), orion.clone());
                    }
                    if let Some(imdb) = &id.imdb {
                        other_attributes.insert("imdb".to_string(), imdb.clone());
                    }
                    if let Some(tmdb) = &id.tmdb {
                        other_attributes.insert("tmdbid".to_string(), tmdb.clone());
                    }
                    if let Some(tvdb) = &id.tvdb {
                        other_attributes.insert("tvdbid".to_string(), tvdb.clone());
                    }
                    if let Some(tvrage) = &id.tvrage {
                        other_attributes.insert("rageid".to_string(), tvrage.clone());
                    }
                    if let Some(trakt) = &id.trakt {
                        other_attributes.insert("trakt_id".to_string(), trakt.clone());
                    }
                    if let Some(slug) = &id.slug {
                        other_attributes.insert("slug".to_string(), slug.clone());
                    }
                    result_type = "series";
                }
                OrionIdRef::Show(id) => {
                    if let Some(orion) = &id.orion {
                        other_attributes.insert("orion_id".to_string(), orion.clone());
                    }
                    if let Some(imdb) = &id.imdb {
                        other_attributes.insert("imdb".to_string(), imdb.clone());
                    }
                    if let Some(tmdb) = &id.tmdb {
                        other_attributes.insert("tmdbid".to_string(), tmdb.clone());
                    }
                    if let Some(tvdb) = &id.tvdb {
                        other_attributes.insert("tvdbid".to_string(), tvdb.clone());
                    }
                    if let Some(tvrage) = &id.tvrage {
                        other_attributes.insert("rageid".to_string(), tvrage.clone());
                    }
                    if let Some(trakt) = &id.trakt {
                        other_attributes.insert("traktid".to_string(), trakt.clone());
                    }
                    if let Some(slug) = &id.slug {
                        other_attributes.insert("slug".to_string(), slug.clone());
                    }
                    result_type = "series";
                }
            }
        }

        let mut peers = 0;

        // Add more attributes as needed, e.g. from stream meta, file, etc.
        if let Some(meta) = meta {
            if let Some(uploader) = &meta.uploader {
                other_attributes.insert("uploader".to_string(), uploader.clone());
            }
            if let Some(release) = &meta.release {
                other_attributes.insert("release".to_string(), release.clone());
            }
            if let Some(edition) = &meta.edition {
                other_attributes.insert("edition".to_string(), edition.clone());
            }
        }
        if let Some(file) = file {
            if let Some(hash) = &file.hash {
                other_attributes.insert("infohash".to_string(), hash.clone());
            }
            if let Some(size) = file.size {
                other_attributes.insert("size".to_string(), size.to_string());
            }
        }
        if let Some(stream_info) = stream_info {
            if let Some(source) = &stream_info.source {
                other_attributes.insert("source".to_string(), source.clone());
            }
            if let Some(seeds) = stream_info.seeds {
                let seeds = seeds.max(10);
                peers = peers.max(seeds);
                other_attributes.insert("seeds".to_string(), seeds.to_string());
            }
            if let Some(hoster) = &stream_info.hoster {
                other_attributes.insert("hoster".to_string(), hoster.clone());
            }
        }
        if let Some(video) = video {
            if let Some(quality) = &video.quality {
                other_attributes.insert("quality".to_string(), quality.clone());
            }
            if let Some(codec) = &video.codec {
                other_attributes.insert("codec".to_string(), codec.clone());
            }
        }
        if let Some(audio) = audio {
            if let Some(audio_codec) = &audio.codec {
                other_attributes.insert("audio_codec".to_string(), audio_codec.clone());
            }
            if let Some(channels) = audio.channels {
                other_attributes.insert("audio_channels".to_string(), channels.to_string());
            }
        }

        other_attributes.insert("peers".to_string(), peers.to_string());

        let links_vec = links.unwrap();
        Some(Torrent {
            title: title.unwrap(),
            description: None,
            result_type: result_type.to_string(),
            size: size.unwrap(),
            category_ids: category_ids.clone(),
            torrent_file_url: links_vec.iter().find(|l| l.ends_with(".torrent")).cloned(),
            magnet_uri: links_vec.iter().find(|l| l.starts_with("magnet:")).cloned(),
            other_attributes: Some(other_attributes),
        })
    }).collect()
}

#[derive(Debug)]
pub struct TorznabSearchHandler {}

impl TorznabSearchHandler {
    pub fn new(orionoid_client: Arc<OrionoidRequestClient>) -> Self {
        ORIONOID_API_CLIENT.set(orionoid_client).expect("Failed to set Orionoid API Client");
        Self {}
    }

    pub async fn search_orionoid(&self, params: torznab::types::SearchParameters) -> Result<Vec<Torrent>, String> {
        let api_token = match params.apikey.as_deref() {
            Some(token) => token,
            None => return Err("API key is required for search".to_string()),
        };
        let client = get_orionoid_client();
        let max_results = params.limit;

        let results = match params.search_type.as_str() {
            "search" => {
                client
                    .search_endpoints()
                    .search_movie(
                        api_token,
                        params.q.as_deref(),
                        params.imdbid.as_deref(),
                        max_results,
                    )
                    .await
            }
            "movie" => {
                client
                    .search_endpoints()
                    .search_movie(
                        api_token,
                        params.q.as_deref(),
                        params.imdbid.as_deref(),
                        max_results,
                    )
                    .await
            }
            "tvsearch" => {
                client
                    .search_endpoints()
                    .search_tv(
                        api_token,
                        params.q.as_deref(),
                        params.imdbid.as_deref(),
                        params.season,
                        params.ep,
                        max_results,
                    )
                    .await
            }
            _ => return Err("Unsupported search type".to_string()),
        };

        match results {
            Ok(response) => Ok(map_orion_api_response_to_torrents(response)),
            Err(e) => Err(format!("Orionoid search failed: {:?}", e)),
        }
    }
}