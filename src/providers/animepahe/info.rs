use crate::models;
use crate::providers::gogoanime::URL;

use std::error::Error;

use anigo;
use ureq;

use serde_json;

pub fn handler(id: String) -> Result<Box<dyn anigo::Info>, Box<dyn Error>> {
    let mut url = URL.to_string();

    url.push_str("info/");
    url.push_str(&id.to_string());

    let body = ureq::get(url.as_str())
        .set("Content-Type", "application/json")
        .set("Accept", "application/json")
        .call()?
        .into_string()?;

    let novel: models::Info = serde_json::from_str(&body)?;
    let data: Box<dyn anigo::Info> = Box::new(novel);

    Ok(data)
}

#[cfg(test)]
#[test]
fn test() -> Result<(), Box<dyn Error>> {
    let _ = handler("komi-san-wa-comyushou-desu-2nd-season-dub".to_string())?;

    Ok(())
}
