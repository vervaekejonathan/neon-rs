use crate::config;
use tungstenite::{Message};
use url::Url;
use log::{warn};
use std::sync::mpsc::{Receiver};

pub fn connect(rx: Receiver<String>) {
    let mut priority_list : Vec<config::List> = config::decode();
    loop {
        match rx.recv() {
            Ok(data) => {
                let (mut socket, _) =
                tungstenite::connect(Url::parse("ws://localhost:9002/socket").unwrap()).expect("Can't connect");                                               
                priority_list.iter_mut().for_each(|state|{
                    if state.name.eq(&data) {
                        let send_msg = Message::Text(state.value.to_string());
                        socket.write_message(send_msg.clone()).unwrap();                

                    }
                });
                socket.close(None).unwrap();
            }
            Err(e) => {
                warn!("{}", e);
            }
        }
    }
}