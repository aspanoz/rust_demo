pub mod csound;
pub mod params;

use bincode::{config, Decode, Encode};
use params::Item;

/// Состояния кнопки
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum ItemState {
   /// кнопкa не используется в раскладке
   Disabled = 0,
   /// кнопкa задействована в текущей раскладке
   Ready,
   /// кнопкa активирована, запущены указаные в раскладке процессы
   Active,
}

/// IPC пакет обновления гуи
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct ItemICP {
   /// id элемента раскладки
   pub id: Item,
   /// статус элемента
   pub status: ItemState,
   /// значение параметра (процент)
   pub value: u8,
}

impl ItemICP {
   pub fn new(id: Item, status: ItemState, value: u8) -> Self {
      Self { id, status, value }
   }
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct UpdateGUI(pub Vec<ItemICP>);

impl UpdateGUI {
   /// преобразовать данные в вектор
   pub fn encode(&self) -> Vec<u8> {
      bincode::encode_to_vec(self, config::standard()).unwrap()
   }

   /// преобразовать вектор в данные
   pub fn decode(encoded: &[u8]) -> Self {
      let (items, _size) = bincode::decode_from_slice(&encoded[..], config::standard()).unwrap();
      items
   }
}
