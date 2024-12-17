use crate::handlers::errors;
use reqwest;
use colored::*;
use roxmltree::Document;

pub async fn manager(url: String) {
    let url = url + "?restype=container&comp=list";
    println!("  \\_ Container URL: {}", url.blue());

    match reqwest::get(&url).await {
        Ok(response) => {
            let body = response.text().await.expect("Failed to get body");

            match errors::handler(&body, &url).await {
                Ok(Some(retry_response)) => {
                    let retry_body: String = retry_response.text().await.expect("Failed to get retry body");
                    xml_handler(&retry_body)
                }
                Ok(None) => {
                    xml_handler(&body);
                }
                Err(e) => {
                    println!(" \\_ Error in handler: {}", e);
                }
            }
        }
        Err(e) => {
            println!(" \\_ Error: {}", e);
        }
    }
}

fn xml_handler(content: &str) {

    let doc = match Document::parse(content) {
        Ok(d) => d,
        Err(_e) => {
            println!("{}", "\n* Error when trying to read the XML content".red());
            return;
        }
    };

    let blobs: Vec<_> = doc.descendants().filter(|node| node.is_element() && node.tag_name().name() == "Blob").collect();

    if blobs.is_empty() {
        println!("{}", "\n* Impossible to find <Blob> tag on the XML".red());
    }

    for blob in blobs {

        let file_name = blob.descendants().find(|n| n.is_element() && n.tag_name().name() == "Name")
        .and_then(|n|n.text()).unwrap_or("N/A").trim().to_string();

        let last_modified = blob.descendants().find(|n| n.is_element() && n.tag_name().name() == "Last-Modified")
        .and_then(|n|n.text()).unwrap_or("N/A").trim().to_string();

        let content_length = blob.descendants().find(|n| n.is_element() && n.tag_name().name() == "Content-Length")
        .and_then(|n|n.text()).unwrap_or("N/A").trim().to_string();

        println!("
* File Name: {}
 \\_ Last-Modified: {}
 \\_ Content-Length: {}
        ", file_name.to_string().blue(), last_modified.to_string().blue(), content_length.to_string().blue());

    }

}