/*
 * A websocket experiment for listening to two servers and
 * combining their  messages into one ws connection for a
 * client to consume
 */
extern crate toml;
extern crate serde;
extern crate ws;

#[macro_use]
extern crate serde_derive;

//use ws::listen;
use std::io::prelude::*;
use std::fs::File;
use ws::{connect, CloseCode, Handler, Sender, Handshake, Result, Message};

struct Client {
    out: Sender,
}
struct Connection {
    ip: String,
    port: String,
}
#[derive(Debug, Deserialize)]
struct Config {
    clients: Option<Vec<ClientConfig>>,
}
#[derive(Debug, Deserialize)]
struct ClientConfig {
    ip: Option<String>,
    port: Option<String>,
}


impl Handler for Client {

    /*
     * Only called after the handshake was successful
     */
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("successfully connected!");
        self.out.send("hello")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("got message: {}", msg);
        self.out.close(CloseCode::Normal)
    }
}

fn main() {
    // load configs
    let mut file = File::open("./config.toml").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    //println!("{}", contents);
    let decoded: Config = toml::from_str(&contents).unwrap();
    //let client1 = decoded.clients;
    println!("{:#?}", decoded);
    //println!("client: {}", client1.ip.unwrap());


    let touchbar = Connection {
        ip: "192.168.1.129".to_string(),
        port: "8001".to_string(),
    };
    let mut address = String::new();
    address = format!("ws://{}:{}", touchbar.ip, touchbar.port);
    println!("Address for the touchbar is: {}", address);

    //connect to the first server
    //connect(address, |out| Client {out: out} ).unwrap();

}

    /*
    // simple websocket server
    listen("127.0.0.1:8001", |out| {
        move |msg| {
            out.send(msg)
            //println!("Got msg: {}", msg);
        }
    }).unwrap()
    */
