use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tide::log::*;
use tokio::runtime::Runtime;
use tokio::signal;
use tokio::sync::mpsc;

mod app;
pub mod config;
pub mod store;

use app::{tray, web, Code};
use config::{Config, CONFIG_PATH};

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0, // 0x0100 on windows
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn run() -> Result<()> {
    pretty_env_logger::init();

    let (_shutdown_send, mut shutdown_recv): (
        mpsc::UnboundedSender<Code>,
        mpsc::UnboundedReceiver<Code>,
    ) = mpsc::unbounded_channel();

    let (tx_main, rx_main) = std::sync::mpsc::channel();
    let main_thread = std::thread::spawn(move || {
        let runtime = Runtime::new().expect("Unable to create the runtime handle");
        tx_main
            .send(runtime.handle().clone())
            .expect("Unable to send runtime handle");

        runtime.block_on(async move {
	      tokio::select! {
	          _ = signal::ctrl_c() => info!("shutting down by ctrl+c signal"),
	          Some(Code::AppTerminate) = shutdown_recv.recv() => info!("shutting down by event trigger")
	      }
	   })
    });

    let config = Config::from_path(CONFIG_PATH).expect("Unable to load config from file");
    let state = Arc::new(Mutex::new(store::State {
        media: store::State::new(&config.library),
        connections: HashMap::new(),
        next_id: 0,
        config,
    }));
    let state_1 = Arc::clone(&state);

    let tray_service = ksni::TrayService::new(tray::App::default());

    let tasks_pool = std::thread::spawn(move || {
        let runtime = rx_main.recv().expect("Unable to recive the runtime handle");

        let web_app = runtime.spawn(async move { web::run(state_1).await });
        tray_service.spawn();
        runtime.block_on(async move { futures::future::join_all(vec![web_app]).await })
    });

    tasks_pool.join().expect("Tasks thread pool panicked");
    main_thread.join().expect("Main runtime thread panicked");

    info!("all tasks terminated");

    Ok(())
}
