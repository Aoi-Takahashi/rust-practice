use reqwest::{Client, Error, RequestBuilder, Url};
use serde::{Deserialize, Serialize};
use std::io;
use url::ParseError;

#[derive(Serialize, Deserialize, Debug)]
struct ResultObject {
    result: bool,
    remain_retry_count: i32,
}

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
    println!("Issue!");
    let get_theme_url = create_url("/questions".to_string()).unwrap();
    let request_builder: RequestBuilder = client.get(get_theme_url);
    let body = request_builder.send().await?.text().await?;
    loop {
        print!("{}", &body);
        println!("Enter the answer! (press Enter after typing)");
        let answer = read_buffer();
        let post_answer_url = create_url("/answers".to_string()).unwrap();
        let request_builder: RequestBuilder = client.post(post_answer_url).body(answer);
        let body: ResultObject = request_builder.send().await?.json().await?;
        match body.result {
            true => {
                println!("Correct!");
                return Ok(());
            }
            false => {
                if body.remain_retry_count == 0 {
                    println!("Incorrect! you are lose");
                    return Ok(());
                }
                println!("Incorrect! Retry count: {}", body.remain_retry_count);
            }
        }
    }
}
