use borovylo_data::DataSchema;
use std::task::Poll::{self, Pending};
use stick::{Controller, Listener, Remap};
// use tokio::sync::oneshot;

use std::error::Error;
// use init::MAX_LAYOUT;

mod event;
mod get_action;
pub use podulo::edit_param::EditParam;

pub type Exit = ();

pub struct State {
    pub listener: Listener,
    pub controllers: Vec<Controller>,
    // pub shutdown_tx: oneshot::Sender<()>,
    rumble: (f32, f32),
    schema: DataSchema,
    // текущие значения
    pid: Option<EditParam>, // параметр
    lid: u8,                // раскладка
                            // chn: u8,                // голос (midi канал)
}

impl State {
    pub fn new(
        schema: DataSchema,
        // shutdown_tx: oneshot::Sender<()>,
    ) -> Result<Self, Box<dyn Error>> {
        let sdb = include_str!("../remap/remap.sdb");
        let remap = Remap::new().load(sdb).unwrap();

        Ok(State {
            listener: Listener::new(remap),
            controllers: Vec::new(),
            rumble: (0.0, 0.0),
            schema,
            // shutdown_tx,
            pid: None,
            lid: 0,
        })
    }

    pub fn connect(&mut self, controller: Controller) -> Poll<Exit> {
        if controller.name() == "Sony Interactive Entertainment Wireless Controller" {
            // info!("connected {:?} {}", controller.name(), controller.id());
            self.controllers.push(controller);
        }
        Pending
    }
}
