use async_graphql::{Context, Enum, Error, Object, Result};
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};

use super::{
    midi::{MidiAction::*, OnMidiAction},
    Storage,
};
use crate::subscribe::Manager;

// const EDIT_PARAM: u8 = 2;
// const NOTE_ON: u8 = 0x90; // 144
const MIDI_CC: u8 = 0xB0; // 176

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Param {
    pub id: u8,
    pub label: String,
}

#[Object]
impl Param {
    async fn id(&self) -> &u8 {
        &self.id
    }

    async fn label(&self) -> &String {
        &self.label
    }
}

impl Param {
    pub fn new<A, B>(id: A, label: B) -> Self
    where
        A: std::convert::Into<u8>,
        B: std::convert::Into<String>,
    {
        return Param {
            id: id.into(),
            label: label.into(),
        };
    }
}

// query, возвращает параметр по индексу
pub async fn get_param_by_id(ctx: &Context<'_>, id: usize) -> Option<Param> {
    ctx.data_unchecked::<Storage>()
        .lock()
        .await
        .params
        .get(id)
        .cloned()
}

pub async fn set_edit_param_by_id(ctx: &Context<'_>, pid: usize) -> Result<usize> {
    let mut db = ctx.data_unchecked::<Storage>().lock().await;

    let prev = db.pid;

    if db.params.contains(pid) {
        db.pid = Some(pid);
    }

    match db.pid {
        // если нет изменений
        next if next == prev => {}

        // оповещаю о смене параметра
        Some(_) => {
            // if is_notice {
            //     Manager::publish(OnMidiAction {
            //         mutation: ActivateParam,
            //         midi: Some([NOTE_ON | db.tid, ActivateParam as u8, pid.clone() as u8 + 1]),
            //     });
            // }

            Manager::publish(Mutation {
                mutation: Event::Select,
                id: pid as u8,
            });
        }

        _ => {}
    };

    Ok(pid)
}

// mutation, установаить параметр для редактирования
pub async fn set_speed(ctx: &Context<'_>, id: u8, speed: u8) -> Result<u8, Error> {
    let db = ctx.data_unchecked::<Storage>().lock().await;

    match speed {
        0 => Manager::publish(OnMidiAction {
            mutation: ParamWaiting,
            midi: None,
        }),

        speed => Manager::publish(OnMidiAction {
            mutation: ParamUpdate,
            midi: Some([MIDI_CC | db.tid, 1 + id as u8, speed]),
        }),
    };

    Ok(id)
}

#[derive(Enum, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Event {
    Select,
    Updated,
}

// mutation event
#[derive(Clone, Debug)]
pub struct Mutation {
    pub mutation: Event,
    pub id: u8,
}

#[Object]
impl Mutation {
    async fn mutation(&self) -> Event {
        self.mutation
    }

    async fn id(&self) -> &u8 {
        &self.id
    }

    async fn item(&self, ctx: &Context<'_>) -> Option<Param> {
        let db = ctx.data_unchecked::<Storage>().lock().await;
        match self.mutation {
            Event::Select => db.params.get(self.id.into()).cloned(),
            _ => None,
        }
    }
}

// (id, Event::Updated) => match &db.edit {
//     None => db.params.get(id.into()).cloned(),
//     param => param.clone(),
// },

// subscribe, подпись на изменения кнопок
pub async fn subscribe(mutation: Option<Event>) -> impl Stream<Item = Mutation> {
    Manager::<Mutation>::subscribe().filter(move |event| {
        let res = if let Some(mutation) = mutation {
            event.mutation == mutation
        } else {
            true
        };
        async move { res }
    })
}
