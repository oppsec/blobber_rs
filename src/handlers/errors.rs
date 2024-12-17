// errors.rs
use std::collections::HashMap;
use colored::*;
use reqwest::{self, Response};

pub async fn handler(content: &str, url: &str) -> Result<Option<Response>, reqwest::Error> {
    let mut errors_list = HashMap::new();
    errors_list.insert("FeatureVersionMismatch", "Trying to add 'x-ms-version: 2020-04-08' header on the request.");
    errors_list.insert("AccountIsDisabled", "The specified account is disabled.");
    errors_list.insert("ResourceNotFound", "Invalid container or blob file specified in the URL.");
    errors_list.insert("InvalidQueryParameterValue", "Invalid container or blob file specified in the URL.");

    for (key, message) in &errors_list {
        if content.contains(key) {
            if *key == "FeatureVersionMismatch" {
                println!("   \\_ {}", message.yellow());

                let client = reqwest::Client::new();
                let res = client
                    .get(url)
                    .header("x-ms-version", "2020-04-08")
                    .send()
                    .await?;

                if res.status().is_success() {
                    println!("    \\_ {}", "Retried with x-ms-version header and succeeded!".green());
                    return Ok(Some(res));
                } else {
                    println!("* Retry with 'x-ms-version' failed with status: {}", res.status().to_string().red());
                }

            } else {
                println!("  \\_ {}", message.yellow());
            }
        }
    }

    Ok(None)
}