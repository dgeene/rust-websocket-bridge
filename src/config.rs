extern crate toml;
extern crate serde;


use std::fmt;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub clients: Option<Vec<ClientConfig>>,
}
#[derive(Debug, Deserialize)]
pub struct ClientConfig {
    pub ip: Option<String>,
    pub port: Option<String>,
}

// TODO I want the damn value
impl fmt::Display for ClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //let mut ip = String::new();
        //ip = self.ip.take();
        write!(f, "({:?}:{:?})", self.ip, self.port)
    }
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
