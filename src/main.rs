use config::Config;
pub mod config;

fn main() {
    let config = Config::new();
    println!("{:#?}", config);
}
