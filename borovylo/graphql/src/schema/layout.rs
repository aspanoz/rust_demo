use super::{button::Button, Storage};
use crate::subscribe::Manager;
use async_graphql::{Context, Enum, Object, Result, ID};
use futures::{Stream, StreamExt};

#[derive(Clone, Debug)]
pub struct Layout {
    pub buttons: Vec<Button>,
    pub color: String,
    pub label: String,
}

// инициалтзация базы данных
impl Default for Layout {
    fn default() -> Self {
        Layout {
            label: "New layout".to_string(),
            color: "#343c46".to_string(),
            buttons: vec![],
        }
    }
}

#[Object]
impl Layout {
    async fn label(&self) -> String {
        self.label.clone()
    }

    async fn color(&self) -> String {
        self.color.clone()
    }

    async fn id(&self, ctx: &Context<'_>) -> usize {
        let db = ctx.data_unchecked::<Storage>().lock().await;
        db.lid
    }

    // query, возвращает все кнопки
    pub async fn buttons(&self, ctx: &Context<'_>) -> Vec<Button> {
        super::button::get_buttons(ctx).await
    }
}

impl Layout {
    pub fn new<A>(label: A, buttons: Vec<Button>) -> Self
    where
        A: std::convert::Into<String>,
    {
        Self {
            label: label.into(),
            buttons,
            color: "#343c46".to_string(),
        }
    }
}

// mutation, загрузить раскадку
pub async fn set_layout_by_id(
    mut db: futures::lock::MutexGuard<'_, super::Data>,
    id: usize,
) -> Result<usize> {
    db.lid = id;
    if let Some(layout) = db.layouts.get(id) {
        db.buttons = Button::init(layout);
    };

    Manager::publish(Mutation::new(id, Event::Updated));
    Ok(id)
}

#[derive(Enum, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Event {
    Updated,
}

// mutation event
#[derive(Clone, Debug)]
pub struct Mutation {
    pub mutation: Event,
    pub id: ID,
    pub layout: Option<Layout>,
}

impl Mutation {
    pub fn new<A>(id: A, mutation: Event) -> Self
    where
        A: std::convert::Into<ID>,
    {
        Mutation {
            id: id.into(),
            mutation,
            layout: None,
        }
    }
}

#[Object]
impl Mutation {
    async fn mutation(&self) -> Event {
        self.mutation
    }

    async fn id(&self) -> &ID {
        &self.id
    }

    async fn item(&self, ctx: &Context<'_>) -> Vec<Button> {
        super::button::get_buttons(ctx).await
    }

    async fn layout(&self, ctx: &Context<'_>) -> Option<Layout> {
        match (self.id.parse::<usize>(), self.mutation) {
            (Ok(id), Event::Updated) => ctx
                .data_unchecked::<Storage>()
                .lock()
                .await
                .layouts
                .get(id)
                .cloned(),
            _ => None,
        }
    }
}

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
