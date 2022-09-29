use models::params;
use std::time::Duration;

mod action;
mod layout;

pub static CSOUND: once_cell::sync::Lazy<CsoundChannel> = once_cell::sync::Lazy::new(|| {
   let (sender, receiver) = crossbeam_channel::unbounded::<PlayInstr>();
   CsoundChannel { sender, receiver }
});

pub static GUI: once_cell::sync::Lazy<GuiChannel> = once_cell::sync::Lazy::new(|| {
   let (sender, receiver) = crossbeam_channel::unbounded::<Vec<u8>>();
   GuiChannel { sender, receiver }
});

pub static PARAMS: once_cell::sync::Lazy<ParamsChannel> = once_cell::sync::Lazy::new(|| {
   let (sender, receiver) = crossbeam_channel::unbounded::<Vec<f64>>();
   ParamsChannel { sender, receiver }
});

pub struct CsoundChannel {
   pub sender: crossbeam_channel::Sender<PlayInstr>,
   pub receiver: crossbeam_channel::Receiver<PlayInstr>,
}

pub struct GuiChannel {
   pub sender: crossbeam_channel::Sender<Vec<u8>>,
   pub receiver: crossbeam_channel::Receiver<Vec<u8>>,
}

pub struct ParamsChannel {
   pub sender: crossbeam_channel::Sender<Vec<f64>>,
   pub receiver: crossbeam_channel::Receiver<Vec<f64>>,
}

#[derive(Debug)]
pub enum PlayInstr {
   /// имя файла из библиотеки
   LoadSample(String),
   /// увеличить параметр
   AddParamStep(params::Item),
   /// уменьшить параметр
   SubParamStep(params::Item),
   /// выход
   Exit,
}

#[derive(Debug, Clone, Copy)]
pub enum EncoderStep {
   AddMax,
   Add,
   Sub,
   SubMax,
}

#[derive(Debug, Clone, Copy)]
pub struct EditAxis {
   pub time: std::time::SystemTime,
   pub value: f32,
   pub speed: Option<EncoderStep>,
   pub delay: u64,
   pub param: params::Item,
}

impl EditAxis {
   pub fn new(value: f32, time: std::time::SystemTime, param: params::Item) -> EditAxis {
      EditAxis {
         value,
         speed: EditAxis::speed(value),
         time,
         delay: 14,
         param,
      }
   }

   // определение шага
   pub fn speed(value: f32) -> Option<EncoderStep> {
      match value {
         d if (0.85..1.01).contains(&d) => Some(EncoderStep::AddMax),
         d if (0.2..0.85).contains(&d) => Some(EncoderStep::Add),
         d if (-0.85..-0.2).contains(&d) => Some(EncoderStep::Sub),
         d if (-1.01..-0.85).contains(&d) => Some(EncoderStep::SubMax),
         _ => None,
      }
   }

   // проверка изменений
   pub fn on_change(&mut self, new: f32, ts: std::time::SystemTime) -> EditAxis {
      let delta = {
         if new > self.value {
            new - self.value
         } else {
            self.value - new
         }
      };
      if ts.duration_since(self.time).unwrap() > Duration::from_millis(self.delay) && delta > 0.08 {
         EditAxis {
            time: ts,
            value: new,
            speed: EditAxis::speed(new),
            delay: self.delay,
            param: self.param,
         }
      } else {
         *self
      }
   }
}

/// данные
pub struct Db {
   /// правый стик, ось oY
   pub right_y: Option<EditAxis>,
   /// раскладки
   pub layouts: std::collections::HashMap<params::Item, layout::ControlMap>,
   /// параметры
   pub params: std::collections::HashMap<params::Item, params::ParamContext>,
   /// текущая раскладка
   pub control_map: params::Item,
   /// фильтр событий gamepad
   pub control_filter: action::DenyEvent,
}

impl Default for Db {
   fn default() -> Db {
      Db {
         right_y: None,
         layouts: Db::init_layouts(),
         params: params::create(),
         control_map: params::Item::RootLayout,
         control_filter: Default::default(),
      }
   }
}
