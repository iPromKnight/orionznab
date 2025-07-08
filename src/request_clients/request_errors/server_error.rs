use crate::request_clients::request_errors::server_body_error::ServerBodyError;

#[derive(Debug)]
pub struct ServerError {
    pub code: u16,
    pub body: ServerBodyError,
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "server error with code {}: {}", self.code, self.body)
    }
}

impl std::error::Error for ServerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.body)
    }
}