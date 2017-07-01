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
    let clients = foo.clients.unwrap(); //TODO use match to error check
    let ip: Option<String> = clients[0].ip;
   // clients = "foo";
    println!("{:?}", ip);



    //TODO
    //let ref ip = &clients[0];
    //let mut address = String::new();
    //address = format!("ws://{}:", ip);
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
