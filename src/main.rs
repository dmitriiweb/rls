use config::Config;

pub mod config;
pub mod renders;

use renders::Render;

fn main() {
    let config = Config::new();
    if config.list {
        let render = renders::List::new(&config);
        render.render();
    } else {
        let render = renders::Table::new(&config);
        render.render();
    }
}
