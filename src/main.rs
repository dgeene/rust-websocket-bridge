/*
 * A websocket experiment for listening to two servers and
 * combining their  messages into one ws connection for a
 * client to consume
 */
extern crate ws;

//use ws::listen;
use ws::{connect, CloseCode};

struct Connection {
    ip: String,
    port: String,
}


fn main() {
    let touchbar = Connection {
        ip: "192.168.1.129".to_string(),
        port: "8001".to_string(),
    };
    let mut address = String::new();
    address = format!("ws://{}:{}", touchbar.ip, touchbar.port);
    println!("Address for the touchbar is: {}", address);

    //connect to the server
    connect(address, |out| {
        out.send("yo waddup").unwrap();

        move |msg| {
            println!("got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    }).unwrap()


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
