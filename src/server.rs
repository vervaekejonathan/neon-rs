use std::{
    net::{TcpListener, TcpStream},
    thread::spawn,
};

use log::*;
use tungstenite::{accept, handshake::HandshakeRole, Error, HandshakeError, Message, Result};
use std::sync::mpsc::{SyncSender};
use std::process;

fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
    match err {
        HandshakeError::Interrupted(_) => panic!("Bug: blocking socket would block"),
        HandshakeError::Failure(f) => f,
    }
}

fn handle_client(stream: TcpStream, tx : SyncSender<String>) -> Result<()> {
    let mut socket = accept(stream).map_err(must_not_block)?;    
    loop {
        match socket.read_message()? {
            msg @ Message::Text(_) | msg @ Message::Binary(_) => {
                info!("Received: {}", msg.clone());
                socket.write_message(msg.clone())?;
                tx.send(msg.to_string()).unwrap();
            }
            Message::Ping(_) | Message::Pong(_) | Message::Close(_) | Message::Frame(_) => {}
        }
    }
}

pub fn connect(tx : SyncSender<String>) {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap_or_else(|e| {
        println!("Error: something went wrong: {}", e);
        process::exit(1)
    });

    for stream in server.incoming() {
        let cloned_tx = tx.clone();
        spawn(move || match stream {
            Ok(stream) => {
                if let Err(err) = handle_client(stream, cloned_tx) {
                    match err {
                        Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                        e => error!("test: {}", e),
                    }
                }
            }
            Err(e) => error!("Error accepting stream: {}", e),
        });
    }
}