use crate::config::Config;


pub struct Table<'a> {
    config: &'a Config,
}


impl<'a> Table<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn render(&self) {
        println!("Table render");
    }
}
