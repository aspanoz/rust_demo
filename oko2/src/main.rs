use anyhow::Result;
use clap::Parser;
use slint::{self, ComponentHandle, Weak};
use tokio;

mod connect;
mod vars;

slint::include_modules!();

#[derive(Parser, Debug)]
#[clap(about, version)]
struct Opt {
    /// URL of the website
    #[clap(default_value = "127.0.0.1")]
    base_url: String,

    /// Webcam name
    #[clap(default_value = "5000")]
    port: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::parse();

    let ui = ui::App::new();
    std::thread::Builder::new()
        .name("io-runtime".into())
        .spawn({
            let ui = ui.as_weak();
            move || io_runtime_run(ui.clone(), opt).expect("fatal error")
        })?;

    ui.on_quit(move || {
        #[cfg(not(target_arch = "wasm32"))]
        std::process::exit(0);
    });

    ui.run();
    Ok(())
}

fn io_runtime_run(ui: Weak<ui::App>, opt: Opt) -> Result<()> {
    use tokio::*;

    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    rt.block_on(connect::io_run(ui, opt))?;
    rt.shutdown_timeout(time::Duration::from_secs(1));
    Ok(())
}
