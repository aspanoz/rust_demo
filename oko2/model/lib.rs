use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use serde_json::to_string_pretty as json_pretty;
// use rusqlite::{Connection, Result};

pub static GUI: once_cell::sync::Lazy<GuiChannel> = once_cell::sync::Lazy::new(|| {
    let (sender, receiver) = crossbeam_channel::unbounded::<Vec<u8>>();
    GuiChannel { sender, receiver }
});

pub struct GuiChannel {
    pub sender: crossbeam_channel::Sender<Vec<u8>>,
    pub receiver: crossbeam_channel::Receiver<Vec<u8>>,
}

#[derive(Default, Debug, Clone)]
pub struct State {
    pub media: Vec<Item>,
    pub next_id: u32,
    pub connections: HashMap<u32, crossbeam_channel::Sender<Vec<u8>>>,
}

#[derive(Default, Debug, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Code {
    // тригер для остановки программы
    AppTerminate,
    // запрос web-клиента случайной картинки
    GetRandomMedia,
    // переместить картнку за индексом в trash-папку
    MoveMediaToTrash,
    // поставить like картинке
    SetLike,
}

// impl State {
//     pub fn new() -> Self {
//         let conn = Connection::open_in_memory().expect("Unable to open db connection");
//         Self::default()
//         //         let cnf_path = std::path::Path::new(CONFIG_URL);
//         // 				let config = match cnf_path {
//         //             path if path.exists() => serde_json::from_str(
//         //                 &std::fs::read_to_string(&path).expect("Unable to read config file"),
//         //             )
//         //             .expect("Unable to deserialize events"),
//         //             _ => {
//         // 							let cnf = Config { data: "lib.db" };
//         // 							let data = json_pretty(&cnf).expect("Unable to serialize Config")
//         // 					    std::fs::write(cnf_path, data).expect("Unable to write config file");
//         // 							cnf
//         // },
//         // };
//         //         Self {
//         //             config,
//         // 						items: Default::defaut()
//         //         }
//     }
// }
// fn open_my_db() -> Result<()> {
//     let path = "./my_db.db3";
//     let db = Connection::open(path)?;
//     // Use the database somehow...
//     println!("{}", db.is_autocommit());
//     Ok(())
// }
// fn main() -> Result<()> {
//     let conn = Connection::open_in_memory()?;
//     conn.execute(
//         "CREATE TABLE item (
//             id    INTEGER PRIMARY KEY,
//             name  TEXT NOT NULL,
//             data  BLOB
//         )",
//         (), // empty list of parameters.
//     )?;
// 		conn.backup
//     let me = Item {
//         id: 0,
//         name: "Steven".to_string(),
//         data: None,
//     };
//     conn.execute(
//         "INSERT INTO person (name, data) VALUES (?1, ?2)",
//         (&me.name, &me.data),
//     )?;
//     let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
//     let person_iter = stmt.query_map([], |row| {
//         Ok(Person {
//             id: row.get(0)?,
//             name: row.get(1)?,
//             data: row.get(2)?,
//         })
//     })?;
//     for person in person_iter {
//         println!("Found person {:?}", person.unwrap());
//     }
//     Ok(())
// }
