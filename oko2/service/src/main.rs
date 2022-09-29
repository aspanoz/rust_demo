#[macro_use]
extern crate log;
use anyhow::Result;
use clap::Parser;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::signal;
use tokio::sync::mpsc;

mod config;
mod tray;
mod vars;
mod ws;

#[derive(Parser, Debug)]
#[clap(about, version)]
struct Opt {
    /// URL of the host
    #[clap(default_value = "0.0.0.0:5000")]
    base_url: String,

    /// Database
    #[clap(default_value = "media.db")]
    db: String,

    /// Root media folder
    #[clap(default_value = "tmp/gallery")]
    media: String,

    /// Trash folder
    #[clap(default_value = "tmp/trash")]
    trash: String,
}

// use config::{Config, CONFIG_PATH};

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
    // let opt = Opt::parse();
    // println!("{:?}", opt);

    let config = Arc::new(Mutex::new(Opt::parse()));
    let state = Arc::new(Mutex::new(media::State::default()));

    // канал для tray
    // let (_tx_tray, rx_tray) = mpsc::channel(128);

    // канал для shutdown сигнала
    let (_shutdown_send, mut shutdown_recv): (
        mpsc::UnboundedSender<media::Code>,
        mpsc::UnboundedReceiver<media::Code>,
    ) = mpsc::unbounded_channel();

    // MAIN
    let (tx_main, rx_main) = std::sync::mpsc::channel();
    let main_thread = std::thread::spawn(move || {
        let runtime = Runtime::new().expect("Unable to create the runtime handle");
        tx_main
            .send(runtime.handle().clone())
            .expect("Unable to send runtime handle");

        // работает, пока не придёт сигнал shutdown
        // @TODO: ожидание завершения задач
        runtime.block_on(async move {
	      tokio::select! {
	          _ = signal::ctrl_c() => info!("shutting down by ctrl+c signal"),
	          Some(media::Code::AppTerminate) = shutdown_recv.recv() => info!("shutting down by event trigger")
	      }
	   })
    });

    let state_1 = Arc::clone(&state);
    let config_1 = Arc::clone(&config);

    // TASKS POOL
    let tasks_pool = std::thread::spawn(move || {
        let runtime = rx_main.recv().expect("Unable to recive the runtime handle");
        let tray_service = ksni::TrayService::new(tray::App::default());

        // let web_app = runtime.spawn(async move { ws::run(config_1, state_1) });
        let web_app = runtime.spawn(async move { ws::run() });
        let tray_app = runtime.spawn(async move {
            tray_service.run()
            // tray::run(state_2, rx_tray, shutdown_send).await
        });
        // runtime.block_on(async move { futures::future::join_all(vec![web_app, tray_app]).await })
        runtime.block_on(async move { futures::future::join_all(vec![tray_app]).await })
    });

    tasks_pool.join().expect("Tasks thread pool panicked");
    main_thread.join().expect("Main runtime thread panicked");

    info!("all tasks terminated");

    Ok(())
}
