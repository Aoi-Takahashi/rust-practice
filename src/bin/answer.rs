use reqwest::{Client, Error, RequestBuilder};
use rust_sample::utils::{create_url, read_buffer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ResultObject {
    result: bool,
    remain_retry_count: i32,
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
        print!("{}\n", &body);
        println!("Enter the answer! (press Enter after typing)");
        let answer = read_buffer();
        let post_answer_url = create_url("/answers".to_string()).unwrap();
        let request_builder: RequestBuilder = client.post(post_answer_url).body(answer);
        let body: ResultObject = request_builder.send().await?.json().await?;
        match body.result {
            true => {
                println!("Correct! you are win!!");
                return Ok(());
            }
            false => {
                if body.remain_retry_count == 0 {
                    println!("Incorrect! you are lose!!");
                    return Ok(());
                }
                println!("Incorrect! Remain Retry count: {}", body.remain_retry_count);
            }
        }
    }
}
