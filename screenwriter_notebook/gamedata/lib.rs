use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty as json_pretty;

pub const GAME_DATA: &str = "gamedata";
pub const BACKUP: &str = "backup";
pub const EVENTS_PATH: &str = "events.json";
pub const DAYS_PATH: &str = "days.json";

pub const GENERATE_CHARS: &str = "0123456789QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm_-=";

pub enum Files {
  Events,
  Days,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
  pub id:    String,
  pub title: String,
  pub text:  Vec<String>,
  pub chars: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Day {
  pub id:     i32,
  pub events: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct State {
  pub events: Vec<Event>,
  pub days:   Vec<Day>,
}

impl State {
  pub fn new() -> Self {
    let game_data_path = std::path::Path::new(GAME_DATA);
    Self {
      events: match game_data_path.join(EVENTS_PATH) {
        path if path.exists() => {
          serde_json::from_str(&std::fs::read_to_string(&path).expect("Unable to read events file"))
            .expect("Unable to deserialize events")
        }
        _ => Default::default(),
      },

      days: match game_data_path.join(DAYS_PATH) {
        path if path.exists() => {
          serde_json::from_str(&std::fs::read_to_string(&path).expect("Unable to read days file"))
            .expect("Unable to deserialize days")
        }
        _ => Default::default(),
      },
    }
  }

  pub fn generate<S: AsRef<str>>(&self, length: usize, charset: S) -> String {
    let charset_str = charset.as_ref();

    if charset_str.is_empty() {
      panic!("Provided charset is empty! It should contain at least one character");
    }

    let chars: Vec<char> = charset_str.chars().collect();
    let mut result = String::with_capacity(length);

    unsafe {
      for _ in 0..length {
        result.push(*chars.get_unchecked(fastrand::usize(0..chars.len())));
      }
    }

    result
  }

  pub fn new_event(&mut self, title: String, text: String, day: i32) {
    let event_id: String = self.generate(12, GENERATE_CHARS);
    let event_id = format!("lok-{event_id}");

    self.events.push(Event { title, text: vec![text], id: event_id.clone(), chars: Default::default() });
    for item in self.days.iter_mut() {
      if item.id == day {
        item.events.push(event_id.clone());
      }
    }
  }

  pub fn backup(&self, data: &str) {
    let game_data_path = std::path::Path::new(GAME_DATA);
    let root = game_data_path.join(BACKUP);
    for i in (1..7).rev() {
      let file = root.join(format!("{i}.{data}"));
      if file.exists() {
        std::fs::copy(file.clone(), root.join(format!("{}.{data}", i + 1)))
          .expect("Unable to rotate file change history");
      }
      if i == 1 {
        std::fs::copy(game_data_path.join(data), file).expect("Unable to create backup copy");
      }
    }
  }

  pub fn save_events(&self) {
    self.save(Files::Events);
  }

  pub fn save_days(&self) {
    self.save(Files::Days);
  }

  fn save(&self, context: Files) {
    let game_data_path = std::path::Path::new(GAME_DATA);
    let (path, data) = match context {
      Files::Events => {
        self.backup(EVENTS_PATH);
        (game_data_path.join(EVENTS_PATH), json_pretty(&self.events).expect("Unable to serialize events"))
      }

      Files::Days => {
        self.backup(DAYS_PATH);
        (game_data_path.join(DAYS_PATH), json_pretty(&self.days).expect("Unable to serialize days"))
      }
    };

    std::fs::write(path, data).expect("Unable to write data file");
  }
}
