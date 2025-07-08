use axum::{
    extract::{State, Query},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::sync::Arc;
use std::borrow::Borrow;
use std::str;
use xml::writer::{EmitterConfig, XmlEvent};
use crate::torznab::types::*;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SearchForm {
    pub t: Option<String>,
    pub q: Option<String>,
    pub apikey: Option<String>,
    pub cat: Option<String>,
    pub imdbid: Option<String>,
    pub season: Option<u32>,
    pub ep: Option<u32>,
    pub attrs: Option<String>,
    pub extended: Option<u8>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

impl SearchForm {
    fn to_parameters(
        &self,
        conf: impl Borrow<Config>,
        search_type: impl AsRef<str>,
    ) -> SearchParameters {
        let search_type: &str = search_type.as_ref();
        let conf: Config = conf.borrow().clone();

        let split = |string: String| -> Option<Vec<u32>> {
            Some(
                string
                    .split(',')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<u32>>(),
            )
        };

        let categories = self.cat.clone().and_then(split);
        let extended_attribute_names: Option<Vec<String>> = self
            .attrs
            .clone()
            .map(|l| l.split(',').map(|s| s.to_string()).collect());

        let mut extended_attrs = None;
        if self.extended == Some(1) {
            extended_attrs = Some(true);
        }

        let mut limit: u32 = self.limit.unwrap_or(conf.caps.limits.default);
        if limit > conf.caps.limits.max {
            limit = conf.caps.limits.max;
        }
        if limit < 1 {
            limit = 1
        }

        SearchParameters {
            search_type: search_type.to_string(),
            q: self.q.clone(),
            apikey: self.apikey.clone(),
            categories,
            imdbid: self.imdbid.clone(),
            season: self.season.clone(),
            ep: self.ep.clone(),
            attributes: extended_attribute_names.clone(),
            extended_attrs,
            offset: self.offset.clone(),
            limit,
        }
    }
}

// Helper to wrap XML responses
pub struct RawXml<T>(pub T);

impl<T: AsRef<str>> IntoResponse for RawXml<T> {
    fn into_response(self) -> Response {
        (
            [("content-type", "application/xml")],
            self.0.as_ref().to_owned(),
        ).into_response()
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub async fn search(
    State(conf): State<Arc<Config>>,
    Query(form): Query<SearchForm>,
) -> impl IntoResponse {
    search_handler(&conf, form, "search").await
}

pub async fn tv_search(
    State(conf): State<Arc<Config>>,
    Query(form): Query<SearchForm>,
) -> impl IntoResponse {
    search_handler(&conf, form, "tv-search").await
}

pub async fn movie_search(
    State(conf): State<Arc<Config>>,
    Query(form): Query<SearchForm>,
) -> impl IntoResponse {
    search_handler(&conf, form, "movie-search").await
}

pub async fn caps(
    State(conf): State<Arc<Config>>,
) -> impl IntoResponse {
    let buffer = Vec::new();
    let mut writer = EmitterConfig::new().create_writer(buffer);

    writer.write(XmlEvent::start_element("caps")).unwrap();

    let mut element = XmlEvent::start_element("server");
    if let Some(server_info) = &conf.caps.server_info {
        for (key, value) in server_info {
            element = element.attr(key.as_str(), value);
        }
    }
    writer.write(element).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();

    writer
        .write(
            XmlEvent::start_element("limits")
                .attr("max", conf.caps.limits.max.to_string().as_str())
                .attr("default", conf.caps.limits.default.to_string().as_str()),
        )
        .unwrap();
    writer.write(XmlEvent::end_element()).unwrap();

    writer.write(XmlEvent::start_element("searching")).unwrap();
    for item in &conf.caps.searching {
        let available = if item.available { "yes" } else { "no" };
        writer
            .write(
                XmlEvent::start_element(item.search_type.as_str())
                    .attr("available", available)
                    .attr("supportedParams", item.supported_params.join(",").as_str()),
            )
            .unwrap();
        writer.write(XmlEvent::end_element()).unwrap();
    }
    writer.write(XmlEvent::end_element()).unwrap();

    writer.write(XmlEvent::start_element("categories")).unwrap();
    for i in &conf.caps.categories {
        writer
            .write(
                XmlEvent::start_element("category")
                    .attr("id", i.id.to_string().as_str())
                    .attr("name", i.name.as_str()),
            )
            .unwrap();
        for j in &i.subcategories {
            writer
                .write(
                    XmlEvent::start_element("subcat")
                        .attr("id", j.id.to_string().as_str())
                        .attr("name", j.name.as_str()),
                )
                .unwrap();
            writer.write(XmlEvent::end_element()).unwrap();
        }
        writer.write(XmlEvent::end_element()).unwrap();
    }
    writer.write(XmlEvent::end_element()).unwrap();

    if let Some(genres) = &conf.caps.genres {
        writer.write(XmlEvent::start_element("genres")).unwrap();
        for genre in genres {
            writer
                .write(
                    XmlEvent::start_element("genre")
                        .attr("id", genre.id.to_string().as_str())
                        .attr("categoryid", genre.category_id.to_string().as_str())
                        .attr("name", genre.name.as_str()),
                )
                .unwrap();
            writer.write(XmlEvent::end_element()).unwrap();
        }
        writer.write(XmlEvent::end_element()).unwrap();
    }

    if let Some(tags) = &conf.caps.tags {
        writer.write(XmlEvent::start_element("tags")).unwrap();
        for tag in tags {
            writer
                .write(
                    XmlEvent::start_element("tag")
                        .attr("name", tag.name.as_str())
                        .attr("description", tag.description.as_str()),
                )
                .unwrap();
            writer.write(XmlEvent::end_element()).unwrap();
        }
        writer.write(XmlEvent::end_element()).unwrap();
    }

    writer.write(XmlEvent::end_element()).unwrap();
    let result = str::from_utf8(writer.into_inner().as_slice())
        .unwrap()
        .to_string();

    RawXml(result)
}


pub async fn search_handler(
    conf: &Config,
    form: SearchForm,
    search_type: &str,
) -> RawXml<String> {
    let parameters = form.to_parameters(conf.clone(), search_type);
    let buffer = Vec::new();
    let mut writer = EmitterConfig::new().create_writer(buffer);

    writer
        .write(
            XmlEvent::start_element("rss")
                .attr("version", "1.0")
                .attr("xmlns:atom", "http://www.w3.org/2005/Atom")
                .attr("xmlns:torznab", "http://torznab.com/schemas/2015/feed"),
        )
        .unwrap();
    writer.write(XmlEvent::start_element("channel")).unwrap();
    writer
        .write(
            XmlEvent::start_element("atom:link")
                .attr("rel", "self")
                .attr("type", "application/rss+xml"),
        )
        .unwrap();

    writer.write(XmlEvent::start_element("title")).unwrap();
    let mut title_provided = false;
    if let Some(server_info) = &conf.caps.server_info {
        if let Some(title) = server_info.get("title") {
            writer.write(XmlEvent::characters(title)).unwrap();
            title_provided = true;
        }
    }
    if !title_provided {
        writer
            .write(XmlEvent::characters("Orionznab by iPromKnight"))
            .unwrap();
    }
    writer.write(XmlEvent::end_element()).unwrap();

    // Handle errors gracefully
    match (conf.search_handler)(parameters).await {
        Ok(items) => {
            for item in items {
                let torrent_file_url = item.torrent_file_url.clone().unwrap_or_default();
                let magnet_uri = item.magnet_uri.clone().unwrap_or_default();

                if torrent_file_url.is_empty() && magnet_uri.is_empty() {
                    // Optionally: skip or log instead of panic
                    continue;
                }

                writer.write(XmlEvent::start_element("item")).unwrap();

                writer.write(XmlEvent::start_element("title")).unwrap();
                writer.write(XmlEvent::characters(&item.title)).unwrap();
                writer.write(XmlEvent::end_element()).unwrap();

                writer
                    .write(XmlEvent::start_element("description"))
                    .unwrap();
                if let Some(desc) = &item.description {
                    writer.write(XmlEvent::characters(desc)).unwrap();
                }
                writer.write(XmlEvent::end_element()).unwrap();

                writer
                    .write(
                        XmlEvent::start_element("torznab:attr")
                            .attr("size", item.size.to_string().as_str()),
                    )
                    .unwrap();
                writer.write(XmlEvent::end_element()).unwrap();

                for id in item.category_ids {
                    writer
                        .write(
                            XmlEvent::start_element("torznab:attr")
                                .attr("name", "category")
                                .attr("value", id.to_string().as_str()),
                        )
                        .unwrap();
                    writer.write(XmlEvent::end_element()).unwrap();
                }

                writer.write(XmlEvent::start_element("link")).unwrap();
                let mut link_filled = false;
                if let Some(ref attributes) = item.other_attributes {
                    if let Some(tmp) = attributes.get("link") {
                        writer.write(XmlEvent::characters(tmp)).unwrap();
                        link_filled = true;
                    }
                }

                if !link_filled {
                    if let Some(ref url) = item.torrent_file_url {
                        writer.write(XmlEvent::characters(url)).unwrap();
                        writer.write(XmlEvent::end_element()).unwrap();
                        writer
                            .write(
                                XmlEvent::start_element("enclosure")
                                    .attr("url", url)
                                    .attr("length", "0")
                                    .attr("type", "application/x-bittorrent"),
                            )
                            .unwrap();
                        writer.write(XmlEvent::end_element()).unwrap();
                    } else {
                        writer.write(XmlEvent::characters(&magnet_uri)).unwrap();
                        writer.write(XmlEvent::end_element()).unwrap();
                        writer
                            .write(
                                XmlEvent::start_element("enclosure")
                                    .attr("url", &magnet_uri)
                                    .attr("length", "0")
                                    .attr("type", "application/x-bittorrent;x-scheme-handler/magnet"),
                            )
                            .unwrap();
                        writer.write(XmlEvent::end_element()).unwrap();
                    }
                }

                if let Some(ref other_attributes) = item.other_attributes {
                    for (key, value) in other_attributes {
                        writer
                            .write(XmlEvent::start_element("torznab:attr").attr(key.as_str(), value))
                            .unwrap();
                        writer.write(XmlEvent::end_element()).unwrap();
                    }
                }

                writer.write(XmlEvent::end_element()).unwrap();
            }
        }
        Err(e) => {
            // Return a minimal error XML, not RSS
            let error_xml = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?><error>{}</error>"#,
                escape_xml(&format!("Search failed: {e}"))
            );
            return RawXml(error_xml);
        }
    }

    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    let result = str::from_utf8(writer.into_inner().as_slice())
        .unwrap()
        .to_string();

    RawXml(result)
}