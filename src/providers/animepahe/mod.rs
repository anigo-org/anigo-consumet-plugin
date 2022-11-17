pub mod info;
pub mod search;
pub mod source;

use std::error::Error;

use anigo::Cache;

#[derive(Debug)]
#[repr(C)]
pub struct Provider {
    cache: Option<Cache>,
}

impl anigo::Provider for Provider {
    fn info(self, id: String) -> Result<Box<dyn anigo::Info>, Box<dyn Error>> {
        info::handler(id)
    }
    fn source(self, id: String) -> Result<Vec<Box<dyn anigo::Source>>, Box<dyn Error>> {
        source::handler(id)
    }
    fn search(self, query: String) -> Result<Vec<Box<dyn anigo::Novel>>, Box<dyn Error>> {
        search::handler(query)
    }
    fn init(&mut self, cache: Cache) -> Result<(), Box<dyn Error>> {
        self.cache = Some(cache);

        Ok(())
    }
}

pub static URL: &'static str = "https://api.consumet.org/anime/animepahe/";

#[no_mangle]
pub static mut ANIMEPAHE: &'static Provider = &Provider {
    cache: Option::None,
};
