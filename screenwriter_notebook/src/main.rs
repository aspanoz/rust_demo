#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod day;
mod vars;

use std::cell::RefCell;
use std::rc::Rc;

use gamedata::State;
pub(crate) use slint::VecModel;

slint::include_modules!();
pub type DayEvents = Vec<DayEvent>;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
  let app = App::new();

  let state = Rc::new(RefCell::new(State::new()));

  let start_day = 1;
  app.set_day(start_day);

  let init_events = day::get_day_notes(start_day, state.clone());
  let day_view = Rc::new(VecModel::from(init_events));
  app.set_events(day_view.clone().into());

  let init_cal = day::get_summer_data(state.clone());
  let events_in_days = Rc::new(VecModel::from(init_cal));
  app.set_events_in_days(events_in_days.clone().into());

  {
    let app_weak = app.as_weak().unwrap();
    let day_view = day_view.clone();
    let state = state.clone();
    app.on_load_next_day(move || {
      let day = app_weak.get_day();
      if let Some(new_day) = match day {
        1..=91 => Some(day + 1),
        _ => None,
      } {
        app_weak.set_day(new_day);
        day_view.set_vec(day::get_day_notes(new_day, state.clone()));
      };
    });
  }

  {
    let app_weak = app.as_weak().unwrap();
    let day_view = day_view.clone();
    let state = state.clone();
    app.on_load_prev_day(move || {
      let day = app_weak.get_day();
      if let Some(new_day) = match day {
        2..=92 => Some(day - 1),
        _ => None,
      } {
        app_weak.set_day(new_day);
        day_view.set_vec(day::get_day_notes(new_day, state.clone()));
      };
    });
  }

  {
    let state = state.clone();
    app.on_event_note_changed(move |event_id, idx, context| {
      let mut state = state.borrow_mut();
      for item in state.events.iter_mut() {
        if *item.id == String::from(event_id.clone()) {
          item.text[idx as usize] = String::from(context.clone());
        }
      }
    });
  }

  {
    let app_weak = app.as_weak().unwrap();
    let day_view = day_view.clone();
    let state = state.clone();
    app.on_event_note_add(move |event_id| {
      {
        let mut state = state.borrow_mut();
        for item in state.events.iter_mut() {
          if *item.id == String::from(event_id.clone()) {
            item.text.push(String::from("текст"));
          }
        }
      }
      let day = app_weak.get_day();
      day_view.set_vec(day::get_day_notes(day, state.clone()));
    });
  }

  {
    let state = state.clone();
    app.on_event_title_changed(move |event_id, context| {
      let mut state = state.borrow_mut();
      for item in state.events.iter_mut() {
        if *item.id == String::from(event_id.clone()) {
          item.title = String::from(context.clone());
        }
      }
    });
  }

  {
    let state = state.clone();
    app.on_save_game_data(move || {
      let state = state.borrow();
      state.save_events();
      state.save_days();
    });
  }

  {
    let state = state.clone();
    app.on_add_new_event(move |label, context, day| {
      {
        let mut state = state.borrow_mut();
        state.new_event(label.to_string(), context.to_string(), day);
      }
      day_view.set_vec(day::get_day_notes(day, state.clone()));
    });
  }
  app.run();
}
