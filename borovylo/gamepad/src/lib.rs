#[macro_use]
extern crate log;

mod state;

use borovylo_data::DataSchema;
use pasts::Loop;
use tokio::sync::oneshot;

use state::State;

pub async fn event_loop(schema: DataSchema, shutdown_tx: oneshot::Sender<()>) {
    let mut state = State::new(schema).unwrap();

    Loop::new(&mut state)
        .when(|s| &mut s.listener, State::connect)
        .poll(|s| &mut s.controllers, State::on_event)
        // .poll(|s| &mut s.controllers, State::exit)
        .await;

    info!("shutdown");

    shutdown_tx
        .send(())
        .expect("Unable to shutdown runtime thread");
}
