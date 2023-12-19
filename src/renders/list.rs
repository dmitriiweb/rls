use crate::config::Config;
use crate::renders::Render;

pub struct List<'a> {
    config: &'a Config,
}

impl<'a> List<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}
impl<'a> Render for List<'a> {
    fn render(&self) {
        println!("Table render");
    }
}
