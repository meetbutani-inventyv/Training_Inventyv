use std::env;
use config::{Config, ConfigError};

pub fn config_environment() -> Result<Config, ConfigError> {
    let env = env::var("ENV").unwrap_or_else(|_| "devlopment".into());
        
    let config = Config::builder();
    let conf = config.add_source(config::File::with_name("src/resources/config.json"))
        .add_source(config::File::with_name(&format!("src/resources/config-{}.json", env)).required(false))
        .build();
    
    conf
}

pub fn main() {
    let c = config_environment().unwrap();
    println!("{:?}", c.get::<String>("database.url").unwrap());
    println!("{:?}", c.get::<String>("website.url").unwrap());
}