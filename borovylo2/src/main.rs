use async_std::task::sleep;
use pasts::{prelude::*, Join, Loop};
use store;

mod csound;
mod gamepad;
// mod gui;
mod tray;
mod vars;
mod ws;

struct Exit;

struct State<'a> {
   db: store::Db,
   // csound: Vec<f64>,
   gilrs: gilrs::Gilrs,
   on_girls: &'a mut (dyn Notifier<Event = ()> + Unpin),
   // on_gui: &'a mut (dyn Notifier<Event = ()> + Unpin),
}

async fn main(executor: &Executor) {
   let tray_service = ksni::TrayService::new(tray::App::default());

   let gilrs = gilrs::GilrsBuilder::new()
      .build()
      .expect("Unable to create gilrs context");

   let on_girls = &mut Loop::pin(|| sleep(core::time::Duration::from_millis(133)));
   // let on_gui = &mut Loop::pin(|| sleep(core::time::Duration::from_millis(70)));

   let mut state = State {
      db: Default::default(),
      // csound: Default::default(),
      on_girls,
      // on_gui,
      gilrs,
   };

   tray_service.spawn();

   executor.spawn(Box::pin(async {
      std::thread::spawn(move || ws::run());
      println!("Start websocket client listener");
   }));

   executor.spawn(Box::pin(async {
      std::thread::spawn(move || csound::run());
      println!("Start csound orchestra");
   }));

   println!("Start gamepad device listener");
   Join::new(&mut state)
      .on(|s| s.on_girls, State::on_girls)
      // .on(|s| s.on_gui, State::gui_update)
      .await;
}
