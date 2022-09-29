use crate::subscribe::Manager;
use async_graphql::{Enum, Object};
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Enum, Eq, PartialEq, Copy, Clone, Debug)]
#[repr(u8)]
pub enum Event {
    PageUpdate = 1,
    ButtonUpdate,
    LogUpdate,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub id: u8,
    pub bg: Option<String>,
    pub opacity: Option<f32>,
    pub label: Option<String>,
}

#[Object]
impl Block {
    async fn id(&self) -> &u8 {
        &self.id
    }
    async fn opacity(&self) -> &Option<f32> {
        &self.opacity
    }

    async fn bg(&self) -> &Option<String> {
        &self.bg
    }
    async fn label(&self) -> &Option<String> {
        &self.label
    }
}

// mutation event
#[derive(Clone, Debug)]
pub struct Mutation {
    pub mutation: Event,
    pub data: Vec<Block>,
}

impl Mutation {
    pub fn new(mutation: Event, data: Vec<Block>) -> Self {
        Mutation { mutation, data }
    }
}

#[Object]
impl Mutation {
    async fn id(&self) -> u8 {
        self.mutation as u8
    }

    async fn data(&self) -> &Vec<Block> {
        &self.data
    }
}

// subscribe
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
