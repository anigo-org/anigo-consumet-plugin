pub mod providers;
pub mod models;

use anigo::Plugin;

#[no_mangle]
pub static PLUGIN: Plugin = Plugin {
    providers: providers::PROVIDERS,
    workers: &[],
    name: "",
};
