use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerValidationBodyError {
    pub errors: Vec<String>,
}

impl std::error::Error for ServerValidationBodyError {}
impl std::fmt::Display for ServerValidationBodyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "server validation body errors:")?;
        for item in self.errors.iter() {
            write!(f, ", {item}")?;
        }
        Ok(())
    }
}