use super::{
   layout::Action::{EditParam, LoadLayout},
   Db, EditAxis, EncoderStep, PlayInstr, CSOUND, GUI,
};
use gilrs::{Button, Event, EventType, Gilrs};

/// Фильтрация событий gamepad
pub struct DenyEvent {
   pub ignore: Vec<Button>,
}

/// изнорировать список контролов геймпада
pub static IGNORED: once_cell::sync::Lazy<Vec<Button>> = once_cell::sync::Lazy::new(|| {
   vec![
      Button::C,
      Button::Z,
      // Triggers
      Button::LeftTrigger,
      Button::LeftTrigger2,
      Button::RightTrigger,
      Button::RightTrigger2,
      // Menu Pad
      Button::Select,
      Button::Start,
      Button::Mode,
      // Sticks
      Button::LeftThumb,
      Button::RightThumb,
      Button::Unknown,
   ]
});

impl DenyEvent {
   fn new(btn: Button) -> Self {
      Self { ignore: vec![btn] }
   }
}

impl Default for DenyEvent {
   fn default() -> Self {
      Self {
         ignore: Default::default(),
      }
   }
}

impl gilrs::ev::filter::FilterFn for DenyEvent {
   fn filter(&self, ev: Option<Event>, _gilrs: &mut Gilrs) -> Option<Event> {
      match ev {
         Some(Event {
            event: EventType::ButtonPressed(btn, ..) | EventType::ButtonReleased(btn, ..),
            id,
            ..
         }) if self.ignore.contains(&btn) || IGNORED.contains(&btn) => Some(Event::new(id, EventType::Dropped)),
         _ => ev,
      }
   }
}

impl Db {
   /// обработка данных со стриков
   pub fn on_axis(&mut self) {
      match self.right_y {
         // увеличить значение
         Some(EditAxis {
            param,
            speed: Some(EncoderStep::AddMax) | Some(EncoderStep::Add),
            ..
         }) => CSOUND.sender.clone().send(PlayInstr::AddParamStep(param)).unwrap(),

         // уменьшить
         Some(EditAxis {
            param,
            speed: Some(EncoderStep::SubMax) | Some(EncoderStep::Sub),
            ..
         }) => CSOUND.sender.clone().send(PlayInstr::SubParamStep(param)).unwrap(),

         _ => {}
      }
   }

   /// обработка данных с кнопок
   pub fn on_button_pressed(&mut self, id: &gilrs::Button) {
      match self.layouts[&self.control_map].buttons[id] {
         // Редактирование параметра - gamepad stick как encoder
         EditParam(item) => {
            let axis = EditAxis::new(0.0, std::time::SystemTime::now(), item);
            self.right_y = Some(axis);
            // let sender = GUI.sender.clone();
            // sender.send(self.layouts[&self.control_map].update_gui(id)).unwrap();
            self.control_filter = DenyEvent::new(*id);
         }

         // Загрузка новой раскладки
         LoadLayout(name) => {
            // println!("Load layout '{:?}'", name);
            self.right_y = None;
            // установить идентификатор активной раскладки
            self.control_map = name;
            // сбросить фильтры
            self.control_filter = Default::default();
            // gui update
            let sender = GUI.sender.clone();
            sender.send(self.layouts[&self.control_map].update_gui(id)).unwrap();
         }

         _ => {
            println!("unknown action");
         }
      }
   }
}
