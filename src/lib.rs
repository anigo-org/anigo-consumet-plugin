use std::{any::Any, fmt::Error};

use anigo::{Plugin, Provider};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Identify {}

impl Provider for Identify {
    fn request(&mut self, key: String) -> Box<dyn Any + Send + Sync> {
        let boxed: Box<dyn Any + Send + Sync> = Box::new(1);
        boxed
    }

    fn init(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

#[no_mangle]
pub static PROVIDER: Identify = Identify {};

#[no_mangle]
pub static PLUGIN: Plugin = Plugin {
    providers: &[],
    workers: &[],
    name: "",
};
