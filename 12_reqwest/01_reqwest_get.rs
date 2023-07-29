use reqwest;

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let response = reqwest::get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .await?
        .text().await?;
    println!("Response {}", response);


    let client = reqwest::Client::new();
    let response = client
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send().await?
        .text().await?;
    println!("Response from client {}", response);
    return Ok(());
}
