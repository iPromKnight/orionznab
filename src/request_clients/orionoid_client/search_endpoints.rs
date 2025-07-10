use crate::request_clients::request_errors::error::Error;
use crate::request_clients::orionoid_client::orionoid_request_client::{OrionoidRequestClient};
use crate::request_clients::orionoid_client::types::{OrionApiResponse};
use tracing::{debug};

const DEFAULT_MOVIE_QUERY: &str = "the matrix";
const DEFAULT_TV_QUERY: &str = "the flash";

pub struct SearchService<'a> {
    pub(crate) client: &'a OrionoidRequestClient,
}

impl<'a> SearchService<'a> {
    pub async fn search_movie(
        &self,
        api_token: &str,
        query: Option<&str>,
        imdbid: Option<&str>,
        max_results: u32,
    ) -> Result<OrionApiResponse, Error> {
        if api_token.is_empty() {
            return Err(Error::Custom("API key is required".to_string()));
        }
        
        let final_query = Some(query.unwrap_or(DEFAULT_MOVIE_QUERY).to_lowercase().to_string());
        
        let mut url = format!(
            "/?keyapp=FGJKJFEBRHEMRFGSBGDLFPRGED96LJJL&keyuser={api_token}&streamtype=torrent&mode=stream&action=retrieve&type=movie&sortvalue=videoquality&sortorder=ascending",
            api_token = api_token,
        );

        if imdbid.is_some() {
            self.append_query_param(&mut url, "idimdb", &imdbid);
        } else {
            self.append_query_param(&mut url, "query", &final_query);
        }

        url.push_str(&format!("&limitcount={}", max_results));

        let response = self.client.execute_raw::<()>(&url).await?;
        self.handle_orionoid_response(response).await
    }

    pub async fn search_tv(
        &self,
        api_token: &str,
        query: Option<&str>,
        imdbid: Option<&str>,
        season: Option<u32>,
        ep: Option<u32>,
        max_results: u32,
    ) -> Result<OrionApiResponse, Error> {
        if api_token.is_empty() {
            return Err(Error::Custom("API key is required".to_string()));
        }

        let final_query = Some(query.unwrap_or(DEFAULT_TV_QUERY).to_lowercase().to_string());

        let mut url = format!(
            "/?keyapp=FGJKJFEBRHEMRFGSBGDLFPRGED96LJJL&keyuser={api_token}&streamtype=torrent&mode=stream&action=retrieve&type=show",
            api_token = api_token,
        );

        if imdbid.is_some() {
            self.append_query_param(&mut url, "idimdb", &imdbid);
        } else {
            self.append_query_param(&mut url, "query", &final_query);
        }
        self.append_query_param(&mut url, "numberseason", &season);
        self.append_query_param(&mut url, "numberepisode", &ep);

        url.push_str(&format!("&limitcount={}", max_results));


        let response = self.client.execute_raw::<()>(&url).await?;
        self.handle_orionoid_response(response).await
    }

    async fn handle_orionoid_response(
        &self,
        response: reqwest::Response,
    ) -> Result<OrionApiResponse, Error> {
        let body = response.text().await.map_err(Error::from)?;
        debug!("Parsing response: {:?}", body);

        let json: serde_json::Value = serde_json::from_str(&body).map_err(Error::from)?;

        if let Some(result) = json.get("result") {
            if result.get("status") == Some(&serde_json::Value::String("error".to_string()))
                && result.get("type") == Some(&serde_json::Value::String("userkey".to_string()))
            {
                return Err(Error::Unauthorized("Invalid User API Key".to_string()));
            }
        }

        let api_response: OrionApiResponse = serde_json::from_str(&body).map_err(Error::from)?;
        Ok(api_response)
    }

    fn append_query_param<T: ToString>(&self, url: &mut String, key: &str, value: &Option<T>) {
        if let Some(val) = value {
            url.push_str(&format!("&{}={}", key, val.to_string()));
        }
    }
}