use gilrs::Button::*;
use models::{params, ItemICP, ItemState, UpdateGUI};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(u16)]
pub enum Action {
   Empty,
   EditParam(params::Item),
   LoadLayout(params::Item),
}

static BUTTONS: [gilrs::Button; 8] = [North, East, West, South, DPadDown, DPadLeft, DPadRight, DPadUp];

/// Раскладки на gamepad'е
#[derive(Clone)]
pub struct ControlMap {
   pub buttons: HashMap<gilrs::Button, Action>,
}

impl ControlMap {
   /// IPC блок данных
   pub fn update_gui(&self, id: &gilrs::Button) -> Vec<u8> {
      let mut update: Vec<ItemICP> = Default::default();
      for ch in BUTTONS.iter() {
         match self.buttons[ch] {
            // нажата кнопкв id, процесс редактирования
            Action::EditParam(pid) if ch == id => update.push(ItemICP::new(pid, ItemState::Active, 0)),
            // кнопки ch в раскладке, действие - редактирование значения
            Action::EditParam(pid) => update.push(ItemICP::new(pid, ItemState::Ready, 0)),
            // загрузить раскладку
            Action::LoadLayout(lid) => update.push(ItemICP::new(lid, ItemState::Ready, 0)),
            // кнопка не задействована
            _ => update.push(ItemICP::new(params::Item::Empty, ItemState::Disabled, 0)),
         }
      }
      UpdateGUI(update).encode()
   }
}

impl Default for ControlMap {
   fn default() -> ControlMap {
      use gilrs::Button::*;
      use Action::Empty;

      ControlMap {
         buttons: HashMap::from([
            (South, Empty),     // cross
            (East, Empty),      // circle
            (West, Empty),      // square
            (North, Empty),     // triangle
            (DPadLeft, Empty),  // Left,
            (DPadRight, Empty), // Right,
            (DPadDown, Empty),  // Down,
            (DPadUp, Empty),    // Up,
         ]),
      }
   }
}

impl super::Db {
   pub fn init_layouts() -> HashMap<params::Item, ControlMap> {
      use gilrs::Button::*;
      use params::Item::*;
      use Action::*;

      let layouts: HashMap<params::Item, ControlMap> = HashMap::from([
         (EmptyBoxLayout, ControlMap::default()),
         (
            RootLayout,
            ControlMap {
               buttons: HashMap::from([
                  (South, EditParam(Level)),
                  (East, EditParam(Attack)),
                  (West, EditParam(Release)),
                  (North, LoadLayout(SecondLayout)),
                  (DPadDown, LoadLayout(SecondLayout)),
                  (DPadLeft, Action::Empty),
                  (DPadRight, EditParam(Pitch)),
                  (DPadUp, Action::Empty),
               ]),
            },
         ),
         (
            SecondLayout,
            ControlMap {
               buttons: HashMap::from([
                  (South, EditParam(StartPosition)),
                  (East, EditParam(Lenght)),
                  (West, EditParam(Pitch)),
                  (North, LoadLayout(RootLayout)),
                  (DPadDown, LoadLayout(RootLayout)),
                  (DPadLeft, EditParam(Level)),
                  (DPadRight, Action::Empty),
                  (DPadUp, Action::Empty),
               ]),
            },
         ),
      ]);
      layouts
   }
}
