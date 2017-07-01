/*
 * A websocket experiment for listening to two servers and
 * combining their  messages into one ws connection for a
 * client to consume
 */
extern crate ws;

#[macro_use]
extern crate serde_derive;


//use ws::listen;
use ws::{connect, CloseCode, Handler, Sender, Handshake, Result, Message};

mod config;

struct Client {
    out: Sender,
}
struct Connection {
    ip: String,
    port: String,
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
    let foo = config::load_config();
    println!("{:#?}", foo.clients.unwrap()[0].ip);
    //println!("{}", foo[0]);
    //let mut address = String::new();
    //address = format!("ws://{}:{}", touchbar.ip, touchbar.port);


    let touchbar = Connection {
        ip: "192.168.1.129".to_string(),
        port: "8001".to_string(),
    };
    //println!("Address for the touchbar is: {}", address);

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
