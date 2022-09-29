use super::{ui, Weak};
use anyhow::Result;
use models::{params, params::Item, UpdateGUI};
use std::collections::HashMap;

use message_io::network::{NetEvent, Transport};
use message_io::node::{self, NodeEvent};

enum Signal {
    Greet, // This is a self event called every second.
           // Other signals here,
}

pub async fn io_run(ui: Weak<ui::App>) -> Result<()> {
    let controls_list: HashMap<Item, params::ParamContext> = params::create();
    let (handler, listener) = node::split();

    let remote_addr = format!("ws://127.0.0.1:{}/", super::vars::PORT);
    let transport = Transport::Ws;
    let (server_id, _local_addr) = handler.network().connect(transport, remote_addr.clone()).unwrap();

    listener.for_each(move |event| match event {
        NodeEvent::Network(net_event) => match net_event {
            NetEvent::Connected(_, established) => {
                if established {
                    println!("Connected to server at {} by {}", server_id.addr(), transport);
                    // println!("Client identified by local port: {}", local_addr.port());
                    handler.signals().send(Signal::Greet);
                } else {
                    println!("Can not connect to server at {} by {}", remote_addr, transport)
                }
            }
            NetEvent::Accepted(_, _) => unreachable!(), // Only generated when a listener accepts
            NetEvent::Message(_, input_data) => {
                match UpdateGUI::decode(input_data) {
                    UpdateGUI(upd) => {
                        let mut controls: Vec<crate::ui::Control> = Default::default();
                        for item in upd.iter() {
                            controls.push(crate::ui::Control {
                                name: controls_list[&item.id].name.clone().into(),
                                status: item.status.clone() as i32,
                                // установка цвета по типу действия
                                action: match item.id {
                                    // раскладки -> цвет Skin.layout-button
                                    Item::EmptyBoxLayout | Item::RootLayout | Item::SecondLayout => 1,
                                    // редактирование параметров -> цвет Skin.edit-button
                                    _ => 0,
                                },
                                // значение в процентах
                                value: 0,
                            });
                        }
                        ui.upgrade_in_event_loop(move |ui| {
                            ui.set_controls(slint::VecModel::from_slice(&controls));
                        });
                    }
                }
            }

            // let message: FromServerMessage = bincode::deserialize(&input_data).unwrap();
            // match message {
            //    FromServerMessage::Pong(count) => {
            //       println!("Pong from server: {} times", count)
            //    }
            //    FromServerMessage::UnknownPong => println!("Pong from server"),
            // }
            NetEvent::Disconnected(_) => {
                println!("Server is disconnected");
                handler.stop();
            }
        },
        NodeEvent::Signal(signal) => match signal {
            Signal::Greet => {
                let message: Vec<u8> = vec![1, 2, 3];

                let output_data = bincode::encode_to_vec(message, bincode::config::standard()).unwrap();
                handler.network().send(server_id, &output_data);
                // handler.signals().send_with_timer(Signal::Greet, Duration::from_secs(1));
            }
        },
    });
    Ok(())
}
