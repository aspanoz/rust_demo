// use super::vars;
use bincode::config;
use crossbeam_channel::select;
use message_io::network::{Endpoint, NetEvent, SendStatus, Transport};
use message_io::node::{self, NodeEvent};

use std::collections::HashMap;
// use std::net::SocketAddr;
struct ClientInfo {
    // count: usize,
}

enum Signal {
    Update(Endpoint),
    // Close,
}

pub fn run() {
    // pub async fn run(state: Db) -> Result<()> {
    // pub fn run(config: super::Opt, state: media::State) {
    let (handler, listener) = node::split();

    let mut clients: HashMap<Endpoint, ClientInfo> = HashMap::new();
    let _ms = |ms| std::time::Duration::from_millis(ms);

    match handler.network().listen(Transport::Ws, "0.0.0.0:5000") {
        // Ok((_id, _real_addr)) => {}
        Ok((_id, _real_addr)) => println!("Server running at {} by {}", _real_addr, Transport::Ws),
        Err(_) => println!(
            "Can not listening at {} by {}",
            "0.0.0.0:5000",
            Transport::Ws
        ),
    }

    listener.for_each(move |event| match event {
        NodeEvent::Network(net_event) => match net_event {
            NetEvent::Connected(_, _) => (), // Only generated at connect() calls.
            NetEvent::Accepted(endpoint, _listener_id) => {
                // clients.insert(endpoint, ClientInfo { count: 0 });
                clients.insert(endpoint, ClientInfo {});
                println!(
                    "Client ({}) connected (total clients: {})",
                    endpoint.addr(),
                    clients.len()
                );
            }
            NetEvent::Message(endpoint, raw_msg) => {
                let (msg, _): (Vec<u8>, _) =
                    bincode::decode_from_slice(&raw_msg[..], config::standard())
                        .expect("Unable to decode message");
                println!("Signal from {:?}", msg);
                // handler.network().send(endpoint, msg.unwrap().as_slice())
                handler.signals().send(Signal::Update(endpoint));
            }
            NetEvent::Disconnected(endpoint) => {
                clients.remove(&endpoint).unwrap();
                println!(
                    "Client ({}) disconnected (total clients: {})",
                    endpoint.addr(),
                    clients.len()
                );
            }
        },
        NodeEvent::Signal(signal) => {
            match signal {
                Signal::Update(endpoint) => {
                    // println!("NodeEvent::Signal(Update)");
                    let gui_update = media::GUI.receiver.clone();
                    select! {
                       recv(gui_update) -> msg => {
                           match handler.network().send(endpoint, msg.unwrap().as_slice()) {
															SendStatus::Sent => handler.signals().send(Signal::Update(endpoint)),
                              // SendStatus::Sent => {
                              //   handler.signals().send(Signal::Update(endpoint));
                              //   // println!("Update gui");
                              // },
                              _ => {
                                println!("Unable to update gui");
                              }
                           };
                        }
                    };
                } // Signal::Close => println!("NodeEvent::Signal(Close)"),
            };
        }
    });

    println!("ws thread done");
}
