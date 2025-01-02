use reqwest::{Client, Error, RequestBuilder, Url};
use std::io;
use url::ParseError;

fn create_url(path: String) -> Result<Url, ParseError> {
    let base_url: String = "http://127.0.0.1:8080".to_string();
    let url = base_url + &path;
    Url::parse(&url)
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    println!("Welcome to Theme Reasoning!");

    println!("Enter the question! (press Enter after typing)");
    let question = read_buffer();
    let post_theme_url = create_url("/questions".to_string()).unwrap();
    let request_builder: RequestBuilder = client.post(post_theme_url).body(question);
    let body = request_builder.send().await?.text().await?;
    println!("{}", body);

    println!("Enter the correct answer! (press Enter after typing)");
    let answer = read_buffer();
    let post_correct_url = create_url("/corrects".to_string()).unwrap();
    let request_builder: RequestBuilder = client.post(post_correct_url).body(answer);
    let body = request_builder.send().await?.text().await?;
    println!("{}", body);

    //TODO: Implement a hint posting function.
    Ok(())
}
