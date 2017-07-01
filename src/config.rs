extern crate toml;
extern crate serde;


use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Config {
    clients: Option<Vec<ClientConfig>>,
}
#[derive(Debug, Deserialize)]
struct ClientConfig {
    ip: Option<String>,
    port: Option<String>,
}

/*
 * Load the configuration file from the root of the project.
 * TODO error checking
 */
pub fn load_config() -> Config {
    let mut file = File::open("./config.toml").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let decoded: Config = toml::from_str(&contents).unwrap();
    return decoded;//TODO find a way to remove
}
