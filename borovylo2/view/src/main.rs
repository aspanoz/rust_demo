use anyhow::Result;
use slint::{self, ComponentHandle, Weak};
use tokio;

mod connect;
mod vars;

mod ui {
   slint::include_modules!();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let ui = ui::App::new();
   std::thread::Builder::new().name("io-runtime".into()).spawn({
      let ui = ui.as_weak();
      move || io_runtime_run(ui.clone()).expect("fatal error")
   })?;

   ui.on_quit(move || {
      #[cfg(not(target_arch = "wasm32"))]
      std::process::exit(0);
   });

   ui.run();
   Ok(())
}

fn io_runtime_run(ui: Weak<ui::App>) -> Result<()> {
   use tokio::*;

   let rt = runtime::Builder::new_current_thread().enable_all().build()?;

   rt.block_on(connect::io_run(ui))?;
   rt.shutdown_timeout(time::Duration::from_secs(1));
   Ok(())
}
