use reqwest::{Client, Error, RequestBuilder};
use rust_sample::utils::{create_url, read_buffer};

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
