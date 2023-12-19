use crate::config::Config;

pub struct List<'a> {
    config: &'a Config,
}

impl<'a> List<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn render(&self) {
        println!("List render");
    }
}
