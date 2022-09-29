#[macro_use]
extern crate log;

use async_graphql::http::WebSocketProtocols;
use async_graphql::*;
use futures_channel::mpsc;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use midir::{MidiOutput, MidiOutputPort};
use serde_json::{from_str, Value};
use std::time::{Duration, SystemTime};
use tokio::time::timeout;

pub mod edit_param;

use borovylo_data::{schema::midi::MidiAction::*, DataSchema};
use edit_param::EditParam;

use async_graphql::{Error, InputObject};
use serde::{Deserialize, Serialize};

// const MIDI_CC: u8 = 0xB0;  // 176
// const NOTE_OFF: u8 = 0x80; // 128
// const NOTE_ON: u8 = 0x90; // 144

struct RepeatUpdateAction {
    ts: std::time::SystemTime,
    // code: u8,
    // cc: u8,
    // value: u8,
    speed: u8,
    midi: [u8; 3],
}

// impl RepeatUpdateAction {
//     pub fn midi(&mut self) -> [u8; 3] {
//         [self.code, self.cc, self.value]
//     }
// }

#[derive(Serialize, Deserialize, Debug, InputObject)]
struct Action {
    id: u8,
    midi: Option<[u8; 3]>,
}

async fn main(schema: DataSchema) -> Result<(), Error> {
    // создаю MIDI OUT
    let podulo = MidiOutput::new("Pompedullo MIDI").unwrap();

    // все доступные порты для midi соединения
    let out_ports = podulo.ports();

    // let connect_to = "Saffire 6 USB";
    let connect_to = "Midi Through";

    // @TODO: интерактивный выбор, сохранение выбора
    // ищу среди port'ов Midi Through, если ненаход - валю ошибкой
    let connect: &MidiOutputPort = match out_ports.len() {
        0 => return Err("не наден ни один midi port".into()),
        _ => {
            let i = match out_ports
                .iter()
                .position(|p| podulo.port_name(p).unwrap().contains(connect_to))
            {
                Some(port_idx) => port_idx,
                None => {
                    let err_msg = format!("не найден midi port с именем '{}'", connect_to);
                    return Err(err_msg.into());
                }
            };
            // println!("{}", i);
            &out_ports[i]
        }
    };
    // MIDI-коннект
    let mut midi_connect = podulo.connect(connect, "podulo")?;

    let (mut tx, rx) = mpsc::unbounded();
    let mut stream = http::WebSocket::new(schema, rx, WebSocketProtocols::GraphQLWS);

    // подключаю к данным
    let query = r#"{ "type": "connection_init" }"#;
    tx.send(query).await?;
    stream.next().await.unwrap(); // скипую сообщение о подключении

    // подписываю на мутации параметра
    let query = r#"{"id":"1","payload":{"query":"subscription { data: midiAction { id, midi } }"},"type":"start"}"#;
    tx.send(query.into()).await?;

    let mut repeat_job: Option<RepeatUpdateAction> = None;

    loop {
        if let Ok(action) = timeout(Duration::from_millis(12), stream.next()).await {
            let json = from_str::<Value>(&action.unwrap().unwrap_text())?
                .pointer("/payload/data/data")
                .unwrap()
                .clone();

            // @TODO: убрать json, использовать сериализацию async_graphql
            match serde_json::from_value(json).unwrap() {
                Action { id, midi } if midi.is_some() && id == ParamUpdate as u8 => {
                    let midi = midi.unwrap();
                    // midi = [midi[0], midi[1], EditParam::get_cc(midi[2])];
                    let job = RepeatUpdateAction {
                        midi: [midi[0], midi[1], EditParam::get_cc(midi[2])],
                        speed: midi[2],
                        // cc: midi[1],
                        // code: midi[0],
                        // value: EditParam::get_cc(midi[2]),
                        ts: SystemTime::now(),
                    };
                    midi_connect.send(&job.midi).unwrap();
                    repeat_job = Some(job);
                    debug!("ParamUpdate");
                }

                // по необходимости сбросить изменение параметра, если локально
                // сменилась раскладка или скорость изменения == 0
                Action { id, .. }
                    if repeat_job.is_some()
                        && [ParamWaiting as u8, LoadLayoutLocal as u8].contains(&id) =>
                {
                    repeat_job = None;
                    debug!("ParamWaiting")
                }

                Action { id, midi, .. } if id == LoadLayout as u8 => {
                    repeat_job = None;
                    midi_connect.send(&midi.unwrap()).unwrap();
                    debug!("LoadLayout")
                }

                Action { id, midi, .. } if id == ActivateParam as u8 => {
                    midi_connect.send(&midi.unwrap()).unwrap();
                    debug!("ActivateParam")
                }

                a => info!("ШИТО {:?}", a),
            }
        }

        repeat_job = if let Some(mut action) = repeat_job {
            let ts = SystemTime::now();

            // EditParam::delay - частота отправки midi в зависимости от
            // скорости изменения параметра midi[2]
            if ts.duration_since(action.ts)?.as_millis() >= EditParam::delay_millis(action.speed) {
                debug!("repeat ParamUpdate {:?}", action.midi);
                midi_connect.send(&action.midi)?;
                action.ts = ts;
            }
            Some(action)
        } else {
            None
        };
    }
}

pub async fn run(schema: DataSchema) {
    match main(schema).await {
        Err(e) => error!("{:#?}", e),
        _ => {}
    }
    info!("Midi driver stopped");
}

// match action {
//   Some(a) if a.id == MidiAction::ParamUpdate => info!("ParamUpdate {:?}", a),
//   Some(a) if a.id == MidiAction::ParamWaiting => info!("ParamWaiting {:?}", a),
//   Some(a) => info!("??? {:?}", a),
//   _ => {}
// }
// tokio::time::sleep(Duration::from_millis(50)).await;
// let json = from_str::<Value>(&stream.next().await.unwrap().unwrap_text())?;
// info!("{:?}", json);
// let json = from_str::<Value>(&stream.next().await.unwrap().unwrap_text())?
//     .pointer("/payload/data/data")
//     .unwrap()
//     .clone();
// let data: Action = serde_json::from_value(json).unwrap();
// info!("{:#?}", data);

// if let Some(data) = stream.next().await {
//     let data = serde_json::from_str::<serde_json::Value>(&data.unwrap_text())?;
//     if let Some(data) = data
//         .as_object()
//         .and_then(|x| x.get("payload"))
//         .and_then(|x| x.get("data"))
//         .and_then(|x| x.get("data"))
//         .and_then(|x| x.get("item"))
//     {
// let data: Data = serde_json::from_value::<Data>(data.to_owned().into())?;
// }
// }

// {
//     let (mut tx, rx) = mpsc::unbounded();
//     let mut stream = http::WebSocket::new(schema, rx, WebSocketProtocols::GraphQLWS);

//     // let query = value!({ "type": "connection_init" });
//     // let query = serde_json::to_string(&query)?;
//     let query = r#"{ "type": "connection_init" }"#;
//     tx.send(query).await?;
//     stream.next().await.unwrap(); // скипую сообщение о подключении

//     // let query = value!({
//     //     "type": "start",
//     //     "id": "1",
//     //     "payload": {
//     //         "query": "subscription { data: editParamChanged { item { id } } }"
//     //     },
//     // });
//     // let query = serde_json::to_string(&query)?;

//     let query = r#"{"id":"1","payload":{"query":"subscription { data: editParamChanged { item { id } } }"},"type":"start"}"#;
//     tx.send(query.into()).await?;

//     loop {
//         tokio::time::sleep(Duration::from_millis(10)).await;
//         let data = stream.next().await;

//         let data = serde_json::from_str::<serde_json::Value>(&data.unwrap().unwrap_text()).unwrap();

//         // let state = async_graphql_value::from_value::<ActionState>(resp.data.to_owned())?;
//         info!("{:#?}", data["payload"]["data"]["data"]);
//     }

//     // info!("Midi driver stopped");
//     // Ok(())
// }
