use super::store::Db;
use serde::{Deserialize, Serialize};

pub mod web_view;
pub mod tray;
pub mod web;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Code {
   AppTerminate,
   GetRandomMedia,
   MoveMediaToTrash,
   SetLike,
}
