use std::cell::RefCell;
use std::rc::Rc;

use slint::{ModelRc, SharedString, VecModel};

use super::{DayEvent, DayEvents};

pub fn get_day_notes(day: i32, state: Rc<RefCell<gamedata::State>>) -> DayEvents {
  let day = day as usize - 1;
  let state = state.borrow();
  let mut day_notes: DayEvents = Default::default();

  let mut events = state.events.clone();
  events.retain(|e| state.days[day].events.contains(&e.id));
  for i in events.iter() {
    let tags: Vec<SharedString> = i.chars.iter().map(|e| SharedString::from(e.clone().to_string())).collect();
    let text: Vec<SharedString> = i.text.iter().map(|e| SharedString::from(e.clone().to_string())).collect();

    let new_event = DayEvent {
      id:    i.id.to_string().into(),
      title: i.title.clone().into(),
      text:  ModelRc::new(VecModel::from(text)),
      tags:  ModelRc::new(VecModel::from(tags)),
    };

    day_notes.push(new_event);
  }

  day_notes
}

pub fn get_summer_data(state: Rc<RefCell<gamedata::State>>) -> Vec<i32> {
  let state = state.borrow();
  let mut data: Vec<i32> = vec![0; 92];

  for day in state.days.iter() {
    let cnt = day.events.len();
    if cnt == 0 {
      continue;
    }
    let id = day.id as usize;
    data[id - 1] = cnt as i32;
  }

  data
}
