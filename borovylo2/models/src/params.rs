use bincode::{Decode, Encode};
use std::collections::HashMap;

/// Параметр Боровыла
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct ParamContext {
   /// название параметра, пока без ограничения длины
   pub name: String,
   /// идентификатор параметра
   pub id: Item,
}

impl ParamContext {
   pub fn new<A>(name: A, id: Item) -> Self
   where
      A: std::convert::Into<String>,
   {
      Self { name: name.into(), id }
   }
}

#[derive(Encode, Decode, PartialEq, Debug, Clone, Hash, Eq, Copy)]
pub enum Item {
   // параметры Боровыла
   Level = 0,
   Attack = 19,
   Release = 16,
   Pitch = 5,
   StartPosition = 14,
   Lenght = 15,

   // ничего такого
   Empty = 100,

   // раскладки
   EmptyBoxLayout,
   RootLayout,
   SecondLayout,
}

pub fn create() -> HashMap<Item, ParamContext> {
   use Item::*;

   HashMap::from([
      (Empty, ParamContext::new("", Empty)),
      // раскладки
      (EmptyBoxLayout, ParamContext::new("EMPTY BOX", EmptyBoxLayout)),
      (RootLayout, ParamContext::new("ROOT", RootLayout)),
      (SecondLayout, ParamContext::new("SECOND", SecondLayout)),
      // параметры Боровыла
      (Level, ParamContext::new("LEVEL", Level)),
      (Attack, ParamContext::new("ATTACK", Attack)),
      (Release, ParamContext::new("RELEASE", Release)),
      (Pitch, ParamContext::new("PITCH", Pitch)),
      (StartPosition, ParamContext::new("START", StartPosition)),
      (Lenght, ParamContext::new("RANGE", Lenght)),
   ])
}
