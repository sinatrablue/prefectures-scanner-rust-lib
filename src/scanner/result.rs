use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct ParsingResult {
    pub url: String,
    pub start_date: String,
    pub end_date: String,
    pub title: String,
    pub quote: String,
}

impl ParsingResult {
    pub fn new(url: String, start_date: String, end_date: String, title: String, quote: String) -> ParsingResult {
        ParsingResult { url, start_date, end_date, title, quote }
    }
    pub fn is_same_url(&self, url: &String) -> bool {
        url == &self.url
    }
}

impl Serialize for ParsingResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 5 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("ParsingResult", 5)?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("date", &self.start_date)?;
        state.serialize_field("date", &self.end_date)?;
        state.serialize_field("text", &self.title)?;
        state.serialize_field("quote", &self.quote)?;
        state.end()
    }
}

pub struct ScanResult {
    pub url: String,
    pub results: Vec<ParsingResult>,
}

impl ScanResult {
    pub fn new(url: String, results: Vec<ParsingResult>) -> ScanResult {
        ScanResult { url, results }
    }
}

impl Serialize for ScanResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ScanResult", 2)?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("results", &self.results)?;
        state.end()
    }
}