mod server;
mod client;
mod config;
use log::*;
use std::sync::mpsc;
use std::thread;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let (tx, rx) = mpsc::sync_channel::<String>(10);
    thread::spawn(move || {
        info!("Starting server");
        server::connect(tx);
    });
    thread::spawn(move || {
        info!("Starting client");
        client::connect(rx);
    });
    
    loop {}
}