use async_graphql::{Context, Enum, Object, Result, ID};
use futures::{Stream, StreamExt};
use slab::Slab;

use super::{
    gui,
    layout,
    layout::Layout,
    // param,
    // midi::{MidiAction::*, OnMidiAction},
    Action,
    Storage,
};
use crate::subscribe::Manager;

#[derive(Clone, Debug)]
pub struct Button {
    // какое дейстивие выолняет кнопка
    pub action: Action,
    // параметры (идндексы) для дейстивия
    pub data: Vec<usize>,
    pub color: String,
}

// инициалтзация базы данных
impl Default for Button {
    fn default() -> Self {
        Button {
            action: Action::None,
            data: vec![],
            color: "0.5".to_string(),
        }
    }
}

#[Object]
impl Button {
    async fn action(&self) -> Action {
        self.action.clone()
    }

    async fn color(&self) -> String {
        self.color.clone()
    }

    async fn id(&self) -> Option<usize> {
        // индекс первого параметра
        self.data.clone().pop()
    }

    async fn label(&self, ctx: &Context<'_>) -> String {
        let db = ctx.data_unchecked::<Storage>().lock().await;

        let len = self.data.len();

        match self.action {
            Action::EditParam if len > 0 => db.params[self.data[0]].clone().label,
            Action::LoadLayout if len > 0 => db.layouts[self.data[0]].clone().label,
            Action::SelectChan if len > 0 => format!("CHAN {}", self.data[0] + 1),
            _ => "".to_string(),
        }
    }
}

impl Button {
    pub fn new(data: Vec<usize>, action: Action) -> Button {
        return Button {
            action,
            data,
            color: "0.5".to_string(),
        };
    }

    // инициализация
    pub fn init(layout: &Layout) -> Slab<Button> {
        let mut buttons = Slab::new();

        for settings in layout.buttons.iter() {
            buttons.insert(settings.clone());
        }

        return buttons;
    }
}

// query, возвращает все кнопки
pub async fn get_buttons(ctx: &Context<'_>) -> Vec<Button> {
    ctx.data_unchecked::<Storage>()
        .lock()
        .await
        .buttons
        .iter()
        .map(|(_, btn)| btn)
        .cloned()
        .collect()
}

// query, возвращает кнопку по индексу
pub async fn get_button_by_id(ctx: &Context<'_>, id: usize) -> Option<Button> {
    ctx.data_unchecked::<Storage>()
        .lock()
        .await
        .buttons
        .get(id)
        .cloned()
}

// mutation, изменить кнопку
pub async fn update(ctx: &Context<'_>, id: usize, action: Action) -> Result<bool> {
    let mut db = ctx.data_unchecked::<Storage>().lock().await;
    let button = db.buttons.get_mut(id).map_or_else(
        || Err(async_graphql::Error::new("Button not found")),
        |btn| Ok(btn),
    )?;
    button.action = action;

    Manager::publish(Mutation::new(id, Event::Updated));

    Ok(true)
}

// mutation, изменить кнопку
pub async fn action(ctx: &Context<'_>, id: usize) -> Result<Option<usize>> {
    // let mut db = ctx.data_unchecked::<Storage>().lock().await;
    let button = ctx
        .data_unchecked::<Storage>()
        .lock()
        .await
        .buttons
        .get_mut(id)
        .map_or_else(
            || Err(async_graphql::Error::new("Button not found")),
            |btn| Ok(btn.clone()),
        )?;

    debug!("action {}", button.action);

    let mut db = ctx.data_unchecked::<Storage>().lock().await;
    // const NOTE_ON: u8 = 0x90; // 144
    match button.action {
        Action::LoadLayout if db.layouts.get(button.data[0]).is_some() => {
            db.lid = button.data[0];
            db.pid = None;
            db.buttons = Button::init(&db.layouts[button.data[0]]);

            Manager::publish(layout::Mutation::new(
                button.data[0],
                layout::Event::Updated,
            ));

            let update = vec![gui::Block {
                id: button.data[0] as u8,
                label: Some(db.layouts[button.data[0]].label.clone()),
                bg: Some(db.layouts[button.data[0]].color.clone()),
                opacity: Some(0.5),
            }];
            Manager::publish(gui::Mutation::new(gui::Event::PageUpdate, update));

            Ok(Some(button.data[0]))
        }

        Action::EditParam => {
            let pid = db.params[button.data[0]].id;
            db.pid = Some(pid as usize);
            Manager::publish(Mutation::new(id, Event::Updated));
            Ok(Some(pid as usize))
        }

        Action::SelectChan => {
            db.tid = button.data[0] as u8;
            // Manager::publish(Mutation::new(id, Event::Updated));
            Ok(Some(button.data[0]))
        }

        Action::None => Ok(None),
        _ => Err(async_graphql::Error::new("Unknown button action")),
    }
}

#[derive(Enum, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Event {
    Created,
    Updated,
}

#[derive(Clone, Debug)]
pub struct Mutation {
    pub mutation: Event,
    pub id: ID,
}

impl Mutation {
    pub fn new<A>(id: A, mutation: Event) -> Self
    where
        A: std::convert::Into<ID>,
    {
        Mutation {
            id: id.into(),
            mutation,
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

    async fn item(&self, ctx: &Context<'_>) -> Option<Button> {
        match (self.id.parse::<usize>(), self.mutation) {
            (Ok(id), Event::Updated | Event::Created) => ctx
                .data_unchecked::<Storage>()
                .lock()
                .await
                .buttons
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
