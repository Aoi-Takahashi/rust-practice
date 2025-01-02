pub mod utils {
    use crate::config::API_URL;
    use reqwest::Url;
    use std::io;
    use url::ParseError;

    pub fn create_url(path: String) -> Result<Url, ParseError> {
        let base_url: String = API_URL.to_string();
        let url = base_url + &path;
        Url::parse(&url)
    }
    pub fn read_buffer() -> String {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");
        buffer.trim().to_string()
    }
}

pub mod config {
    pub const API_URL: &str = "http://127.0.0.1:8080";
    pub const IP: &str = "127.0.0.1";
    pub const PORT: u16 = 8080;
}
