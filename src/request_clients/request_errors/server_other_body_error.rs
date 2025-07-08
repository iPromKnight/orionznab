use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerOtherBodyError {
    pub status_code: u16,
    pub status_message: String,
}

impl std::error::Error for ServerOtherBodyError {}
impl std::fmt::Display for ServerOtherBodyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "server body error with code {}: {}",
            self.status_code, self.status_message
        )
    }
}