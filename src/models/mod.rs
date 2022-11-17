use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    #[serde(rename(deserialize = "results"))]
    pub data: Vec<Novel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceResponse {
    #[serde(rename(deserialize = "sources"))]
    pub data: Vec<Source>,
}

//

#[derive(Serialize, Deserialize, Debug)]
pub struct Novel {
    title: String,
    id: String,
}

impl anigo::Novel for Novel {
    fn title(self) -> String {
        self.title
    }
    fn id(self) -> String {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Episode {
    number: u16,
    title: Option<String>,
    id: String,
}

impl anigo::Episode for Episode {
    fn title(self) -> Option<String> {
        self.title
    }
    fn number(self) -> u16 {
        self.number
    }
    fn id(self) -> String {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    #[serde(rename(deserialize = "releaseDate"))]
    release_date: String,
    title: String,
    id: String,
    episodes: Vec<Episode>,
    image: String,
}

impl anigo::Info for Info {
    fn release_date(self) -> Option<String> {
        Option::Some(self.release_date)
    }
    fn image_url(self) -> Option<String> {
        Option::Some(self.image)
    }
    fn episodes(self) -> Vec<Box<dyn anigo::Episode>> {
        let mut data: Vec<Box<dyn anigo::Episode>> = Vec::new();

        for entry in self.episodes {
            data.push(Box::new(entry))
        }

        data
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    quality: String,
    url: String,
}

impl anigo::Source for Source {
    fn quality(self) -> String {
        self.quality
    }
    fn url(self) -> String {
        self.quality
    }
}
