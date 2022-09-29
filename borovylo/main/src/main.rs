extern crate futures;
extern crate tokio;
#[macro_use]
extern crate log;
extern crate async_stream;
extern crate env_logger;
mod logger;
use async_graphql::Schema;

// use async_ctrlc::CtrlC;
use borovylo_data::{MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use logger::color_log_builder;
use tokio::{runtime::Runtime, sync::oneshot};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() {
  std::process::exit(match app() {
    Ok(_) => 0, // 0x0100 on windows
    Err(err) => {
      eprintln!("error: {:?}", err);
      1
    }
  });
}

fn app() -> Result<()> {
  let mut logger: env_logger::Builder = color_log_builder();
  logger.init();

  let (shutdown_tx, shutdown_rx) = oneshot::channel();
  let (handle_tx, handle_rx) = std::sync::mpsc::channel();

  // генератор мастер-хандлера для всех тредов
  let runtime_thread = std::thread::spawn(move || {
    let runtime = Runtime::new().expect("Unable to create the runtime");
    debug!("Runtime created");
    handle_tx
      .send(runtime.handle().clone())
      .expect("Unable to send runtime handle");
    runtime.block_on(async {
      shutdown_rx.await.expect("Error on the shutdown channel");
    });
    info!("Runtime finished");
  });

  // выход по ctrl+c
  // let ctrlc_thread = std::thread::spawn(move || {
  //     let ctrlc_await = Runtime::new().expect("Unable to create the runtime thread");
  //     let signal = CtrlC::new().expect("Cannot create Ctrl+C handler");
  //     debug!("ctrl+c thread created");
  //     ctrlc_await.block_on(async move {
  //         signal.await;
  //         println!("");
  //         info!("Quitting");
  //         shutdown_tx
  //             .send(())
  //             .expect("Unable to shutdown runtime thread");
  //     });
  // });

  let storage = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
    .data(Storage::default())
    .finish();

  let client_state = storage.clone();
  let midi_state = storage.clone();

  let main_thread = std::thread::spawn(move || {
    debug!("Main thread created");
    let handle = handle_rx.recv().expect("Could not recv a handle");
    // инициализация схемы данных
    let main_tasks = vec![
      handle.spawn(async move { jupen::run(client_state).await }),
      handle.spawn(async move { podulo::run(midi_state).await }),
    ];

    handle.block_on(async move {
      futures::future::join_all(main_tasks).await;
    });
    info!("Main thread finished");
  });

  let input_state = storage.clone();
  pasts::block_on(gamepad::event_loop(input_state, shutdown_tx));

  info!("Main thread join");

  // ctrlc_thread.join().expect("ctrl+c thread panicked");
  main_thread.join().expect("Main thread panicked");
  runtime_thread.join().expect("Runtime thread panicked");

  Ok(())
}
