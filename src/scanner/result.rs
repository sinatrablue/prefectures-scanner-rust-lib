pub struct ScanResult {
    pub url: String,
    pub date: String,
    pub text: String,
}

impl ScanResult {
    pub fn new(url: String, date: String, text: String) -> ScanResult {
        ScanResult { url, date, text }
    }
    pub fn is_same_url(&self, url: &String) -> bool {
        url == &self.url
    }
}