use reqwest; // HTTP request
use clap::Parser; // CLI 
use std::{thread, time::Duration};
use url::Url; // Check url format


#[derive(Parser, Debug)]
#[command(author = "mascai", version = "1.0.0", about = "Check interval and URL to check", long_about = None)]
struct Args {
    /// Check interval in seconds
    check_interval: u64,

    /// URL to check
    url: String
}


fn check_url(client: &reqwest::blocking::Client, url: &str) {
    let response = client.get(url).send();
    match response {
        Ok(data) => {
            match data.status() {
                reqwest::StatusCode::OK => println!("Checking '{url}'. Result: OK(200)"), // Example: http://yandex.ru  
                error_code => println!("Checking '{url}'. Result: ERR({})", error_code.as_u16()) // Example: http://httpbin.org/404
            }
            
        },
        Err(error_data) => { // Example: http://yfakeSite2134123412341.com
            let error_code: reqwest::StatusCode = error_data
                .status()
                .unwrap_or(reqwest::StatusCode::from_u16(500).unwrap());
            println!("Checking '{url}'. Result: ERR({})", error_code.as_u16());
        }
    };

    
}

fn main() {
    let args = Args::parse();
    let url = Url::parse(&args.url).expect("URL parsing error"); // Panic in case of invalid format

    let check_interval: u64 = args.check_interval;
    // println!("User innput: {check_interval} {url}");

    let client = reqwest::blocking::Client::new();
    loop {
        check_url(&client, url.as_str());
        thread::sleep(Duration::from_secs(check_interval));
    }
}
