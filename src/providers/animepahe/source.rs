use crate::models;
use crate::providers::gogoanime::URL;

use std::error::Error;

use anigo;
use ureq;

use serde_json;

pub fn handler(id: String) -> Result<Vec<Box<dyn anigo::Source>>, Box<dyn Error>> {
    let mut url = URL.to_string();

    url.push_str("watch/");
    url.push_str(&id.to_string());

    let body = ureq::get(url.as_str())
        .set("Content-Type", "application/json")
        .set("Accept", "application/json")
        .call()?
        .into_string()?;

    let novel: models::SourceResponse = serde_json::from_str(&body)?;
    let mut data: Vec<Box<dyn anigo::Source>> = Vec::new();

    for entry in novel.data {
        data.push(Box::new(entry))
    }

    Ok(data)
}

#[cfg(test)]
#[test]
fn test() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        handler("chuunibyou-demo-koi-ga-shitai-movie-episode-1".to_string())?.len(),
        5
    );

    Ok(())
}
