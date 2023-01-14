use std::{
    net::{TcpListener, TcpStream},
};

use std::thread;
use log::*;
use tungstenite::{accept, handshake::HandshakeRole, Error, HandshakeError, Message, Result};
use std::sync::mpsc::{sync_channel, SyncSender};
use std::process;
use url::Url;


fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
    match err {
        HandshakeError::Interrupted(_) => panic!("Bug: blocking socket would block"),
        HandshakeError::Failure(f) => f,
    }
}

fn handle_client(stream: TcpStream, tx : SyncSender<Message>) -> Result<()> {
    let mut socket = accept(stream).map_err(must_not_block)?;    
    loop {
        match socket.read_message()? {
            msg @ Message::Text(_) | msg @ Message::Binary(_) => {
                info!("Received: {}", msg.clone());
                tx.send(msg).unwrap();
            }
            Message::Ping(_) | Message::Pong(_) | Message::Close(_) | Message::Frame(_) => {}
        }
    }
}

fn connect(tx : SyncSender<Message>) {
    let server = TcpListener::bind("127.0.0.1:9002").unwrap_or_else(|e| {
        println!("Error: something went wrong: {}", e);
        process::exit(1)
    });

    for stream in server.incoming() {
        let cloned_tx = tx.clone();
        thread::spawn(move || match stream {
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


#[test]
fn websocket_echo() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    println!("starting");
    let (tx, rx) = sync_channel::<Message>(10);
    
    thread::spawn(move || {
        info!("Starting server");
        connect(tx);
    });


    let (mut socket_send, _) =
        tungstenite::connect(Url::parse("ws://localhost:9001/socket").unwrap()).expect("Can't connect");
    
    let send_msg = Message::Text("state_1".into());
    socket_send.write_message(send_msg.clone()).unwrap();

    loop {
        match rx.recv() {
            Ok(data) => {
                println!("Receveid: {}", data);
                assert!(data.eq(&Message::Text("red".into())));
                break;
            },
            Err(e) => {
                println!("{}", e);
                assert!(false);
                break;
            }
        }
    }

    socket_send.close(None).unwrap();
}