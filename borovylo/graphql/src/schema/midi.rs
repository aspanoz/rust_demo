use crate::subscribe::Manager;
use async_graphql::{Enum, Object};
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Enum, Eq, PartialEq, Copy, Clone, Debug)]
#[repr(u8)]
pub enum MidiAction {
    LoadLayout = 1,  // загрузить раскладку
    ActivateParam,   // установаить активный параметр
    ParamUpdate,     // менять параметр
    ParamWaiting,    // перестать менять параметр
    LoadLayoutLocal, // загрузить раскладку
}

// mutation event
#[derive(Clone, Debug)]
pub struct OnMidiAction {
    pub mutation: MidiAction,
    pub midi: Option<[u8; 3]>,
}

impl OnMidiAction {
    pub fn new(mutation: MidiAction, midi: Option<[u8; 3]>) -> Self {
        OnMidiAction { mutation, midi }
    }
}

#[Object]
impl OnMidiAction {
    // преобразует midiAction в u8
    async fn id(&self) -> u8 {
        self.mutation as u8
    }

    // готовый миди данные для отправки
    async fn midi(&self) -> &Option<[u8; 3]> {
        &self.midi
    }
}

// subscribe
pub async fn subscribe(mutation: Option<MidiAction>) -> impl Stream<Item = OnMidiAction> {
    Manager::<OnMidiAction>::subscribe().filter(move |event| {
        let res = if let Some(mutation) = mutation {
            event.mutation == mutation
        } else {
            true
        };
        async move { res }
    })
}
