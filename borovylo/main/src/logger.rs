extern crate env_logger;
extern crate log;

use env_logger::{fmt::Color, Builder, Env};
use log::Level;
use std::io::Write;

pub fn color_log_builder() -> Builder {
  let mut builder = Builder::from_env(Env::default().default_filter_or("info"));

  builder.format(|f, record| {
    let target = record.target();
    let mut style = f.style();
    let level = match record.level() {
      Level::Trace => style.set_color(Color::Green),
      Level::Debug => style.set_color(Color::Blue),
      Level::Info => style.set_color(Color::Yellow),
      Level::Warn => style.set_color(Color::Magenta),
      Level::Error => style.set_color(Color::Red),
    };

    writeln!(f, "[{}]: {}", level.value(target), record.args())
  });

  builder
}
