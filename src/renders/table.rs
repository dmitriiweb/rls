use crate::config::Config;
use crate::renders::Render;

pub struct Table<'a> {
    config: &'a Config,
}

impl<'a> Table<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}

impl<'a> Render for Table<'a> {
    fn render(&self) {
        println!("Table render");
    }
}
