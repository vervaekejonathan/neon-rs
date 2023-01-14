use tungstenite::{Message};
use url::Url;
use log::{warn};
use std::sync::mpsc::{Receiver};

pub fn connect(rx: Receiver<String>) {
    loop {
        match rx.recv() {
            Ok(data) => {
                let (mut socket, _) =
                tungstenite::connect(Url::parse("ws://localhost:9002/socket").unwrap()).expect("Can't connect");                               
                let send_msg = Message::Text(data);
                socket.write_message(send_msg.clone()).unwrap();                
                socket.close(None).unwrap();
            }
            Err(e) => {
                warn!("{}", e);
            }
        }
    }
}