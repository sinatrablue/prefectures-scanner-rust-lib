use reqwest::Error;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

pub struct ParsingError {
    url: String,
    error: Error,
}

impl ParsingError {
    pub fn new(url: String, error: Error) -> Self {
        Self { url, error }
    }
}

impl Serialize for ParsingError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ScanResult", 2)?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("errors", &self.error.to_string())?;
        state.end()
    }
}