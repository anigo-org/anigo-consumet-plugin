pub mod animepahe;
pub mod enime;
pub mod gogoanime;

use anigo::Provider;

pub static PROVIDERS: &'static [(&'static str, &'static dyn Provider)] = unsafe {
    &[
        ("animepahe", animepahe::ANIMEPAHE),
        ("enime", enime::ENIME),
        ("gogoanime", gogoanime::GOGOANIME),
    ]
};
