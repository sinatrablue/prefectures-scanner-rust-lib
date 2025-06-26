use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct ScanResult {
    pub url: String,
    pub start_date: String,
    pub end_date: String,
    pub text: String,
}

impl ScanResult {
    pub fn new(url: String, start_date: String, end_date: String, text: String) -> ScanResult {
        ScanResult { url, start_date, end_date, text }
    }
    pub fn is_same_url(&self, url: &String) -> bool {
        url == &self.url
    }
}

impl Serialize for ScanResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("ScanResult", 4)?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("date", &self.start_date)?;
        state.serialize_field("date", &self.end_date)?;
        state.serialize_field("text", &self.text)?;
        state.end()
    }
}