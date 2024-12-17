use blobber_rs::utils::ui;
use blobber_rs::handlers::blob;

use reqwest;
use tokio;
use clap::Parser;
use colored::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    url: String
}

async fn test_connection(url: String) {
    match reqwest::get(&url).await {
        Ok(response) => {
            let initial_status_code = response.status();
            println!(" \\_ Initial status code: {}", initial_status_code.to_string().yellow());
            blob::manager(url).await;
        }
        Err(e) => {
            println!(" \\_ Error: {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    ui::banner();

    let args = Args::parse();
    let url = args.url;

    println!("* Connecting to {}", url.green());

    test_connection(url).await;
}