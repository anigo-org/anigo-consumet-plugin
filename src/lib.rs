pub mod models;
pub mod providers;

use anigo::Plugin;

#[no_mangle]
pub static PLUGIN: Plugin = Plugin {
    providers: providers::PROVIDERS,
    workers: &[],
    name: "",
};
