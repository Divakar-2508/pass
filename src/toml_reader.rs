use toml;
use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u32,
    pub username: String,
    pub password: String,
    pub db: String,
}

#[derive(Deserialize)]
pub struct Data {
    pub database: Database,
}

pub fn initialize() {

}

pub fn toml_read() -> Option<Data>{
    let file = "config.toml";

    let content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("\nError reading the config file: {}", err);
            println!("\nCheck if the 'config.toml' file is in the same directory as the executable");
            return None;
        }
    };

    let data : Data = match toml::from_str(&content) {
        Ok(data) => data,
        Err(err) => {
            println!("\nCan't get the data from toml file");
            println!("Error: {}", err);
            return None;
        }
    };
    
    Some(data)
}