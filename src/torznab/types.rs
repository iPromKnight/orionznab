//! All examples here are based off the [Torznab spec](https://torznab.github.io/spec-1.3-draft/torznab/Specification-v1.3.html)'s `/api?caps` example.
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub(crate) type SearchFunc = Arc<dyn Fn(SearchParameters) -> Pin<Box<dyn Future<Output = Result<Vec<Torrent>, String>> + Send>> + Send + Sync>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Limits {
    /// The maximum number of entries that can be listed in a search query
    pub max: u32,
    /// The default number of entries to be listed in a search query
    pub default: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchInfo {
    /// What type of search this is - must be `search`, `tv-search`, `movie-search`, `audio-search`, or `book-search`
    pub search_type: String,
    /// Whether this search type is available
    pub available: bool,
    /// The supported parameters for this search type
    ///
    /// Highly recommended: `q` (free text query)
    pub supported_params: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subcategory {
    /// The numeric ID of a subcategory
    ///
    /// The (de facto?) standard is `xxyy`, xx being the first two digits of the category, and the last two digits specifying the subcategory; see also: Category
    pub id: u32,
    /// The name of the subcategory, e.g. "Anime" under the "TV" cateogyr
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category {
    /// The numeric ID of a category
    ///
    /// The (de facto?) standard is `xxyy`, xx being the first two digits of the category, and the last two digits specifying the subcategory; see also: Subcategory
    pub id: u32,
    /// The name of the category, e.g. "Movies"
    pub name: String,
    /// A vector of all the subcategory in this category
    pub subcategories: Vec<Subcategory>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Genre {
    /// The numeric ID of a genre
    ///
    /// I'm not aware of any standard for numbering this; the specification for Torznab shows an example with an ID of 1.
    pub id: u32,
    /// The numeric ID of the category this genre is for.
    pub category_id: u32,
    /// The name of the genre
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    /// The name of a tag for a torrent
    pub name: String,
    /// The description of the tag
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Caps {
    /// The server info, like title - optional
    ///
    /// Examples: `version`, `title`, `email`, `url`, `image`
    pub server_info: Option<HashMap<String, String>>,
    /// The max and default number of items to be returned by queries
    pub limits: Limits,
    /// Info about each type of search
    pub searching: Vec<SearchInfo>,
    /// What categories the server has
    pub categories: Vec<Category>,
    /// What genres the server has (optional)
    pub genres: Option<Vec<Genre>>,
    /// What torrents can be tagged with (optional)
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone)]
pub struct Config {
    /// The function to use for all search types
    ///
    /// What search types are available is dependent on what's marked as available in the `searching` field of `caps` ([`Caps`])
    ///
    /// Search types: `search`, `tv-search`, `movie-search`
    pub search_handler: SearchFunc,
    /// The capabilities of the indexer
    pub caps: Caps,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Holds the parameters for a search query
pub struct SearchParameters {
    /// What type of search this is
    ///
    /// Search types: `search`, `tv-search`,
    pub search_type: String,
    /// The text query for the search
    pub q: Option<String>,
    /// The apikey, for authentication
    pub apikey: Option<String>,
    /// A [`Vec`] containing the numeric category IDs to be included in the search results
    pub categories: Option<Vec<u32>>,
    /// The Imdb ID of the item to search for
    pub imdbid: Option<String>,
    /// The season number of the item to search for
    pub season: Option<u32>,
    /// The episode number of the item to search for
    pub ep: Option<u32>,
    /// A [`Vec`] containing the extended attribute names to be included in the search results
    pub attributes: Option<Vec<String>>,
    /// Whether *all* extended attributes should be included in the search results; overrules `attributes`
    pub extended_attrs: Option<bool>,
    /// How many items to skip/offset by in the results.
    pub offset: Option<u32>,
    /// The maximum number of items to return - also limited to whatever `limits` is in [`Caps`]
    pub limit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Torrent {
    /// The title of the torrent
    pub title: String,
    /// The description of the torrent - optional
    pub description: Option<String>,
    /// The size of the torrent, **in bytes**
    pub size: u64,
    /// A vector of (sub)category IDs
    pub category_ids: Vec<u32>,
    /// The URL of the `.torrent` file
    pub torrent_file_url: Option<String>,
    /// The magnet URI o the torrent
    pub magnet_uri: Option<String>,
    /// Any other attributes
    pub other_attributes: Option<HashMap<String, String>>,
}
