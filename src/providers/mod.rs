pub mod enime;
pub mod gogoanime;

use anigo::Provider;

pub static PROVIDERS: &'static [&'static dyn Provider] = unsafe { &[gogoanime::GOGOANIME] };
