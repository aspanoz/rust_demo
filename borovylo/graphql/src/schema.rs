mod button;
pub mod gui;
mod init;
mod layout;
pub mod midi;
mod param;

use super::{Action, MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use async_graphql::{Context, Object, Result, Subscription};
use futures::Stream;
use midi::{MidiAction::LoadLayout, OnMidiAction};
use slab::Slab;

pub struct Data {
    pub layouts: Slab<layout::Layout>,
    pub buttons: Slab<button::Button>,
    pub params: Slab<param::Param>,

    pub pid: Option<usize>, // параметр
    pub lid: usize,         // раскладка
    pub tid: u8,            // track (midi ch)
}

// инициалтзация базы данных
impl Default for Data {
    fn default() -> Self {
        let layouts = layout::Layout::init();
        let lid: usize = 0;
        Data {
            buttons: button::Button::init(layouts.get(lid).unwrap()),
            params: param::Param::init(),
            layouts,
            lid,
            pid: None,
            tid: 0,
        }
    }
}

#[Object]
impl QueryRoot {
    // текущее состояние всех кнопок
    async fn get_all_buttons(&self, ctx: &Context<'_>) -> Vec<button::Button> {
        button::get_buttons(ctx).await
    }

    // состояние кнопоки по индексу
    async fn get_button(&self, ctx: &Context<'_>, id: usize) -> Option<button::Button> {
        button::get_button_by_id(ctx, id).await
    }

    // параметр по индексу
    async fn get_param(&self, ctx: &Context<'_>, id: usize) -> Option<param::Param> {
        param::get_param_by_id(ctx, id).await
    }
}

const NOTE_ON: u8 = 0x90; // 144

#[Object]
impl MutationRoot {
    // Мутации кнопок раскладки
    async fn update_button(
        &self,
        ctx: &Context<'_>,
        id: usize,
        action_type: Action,
    ) -> Result<bool> {
        button::update(ctx, id, action_type).await
    }

    // загрузить расклатку по индексу, оповестить боровыло
    async fn set_layout(&self, ctx: &Context<'_>, id: usize) -> Result<usize> {
        let db = ctx.data_unchecked::<Storage>().lock().await;
        if db.lid != id {
            let midi = Some([NOTE_ON | db.tid, LoadLayout as u8, id as u8]);
            debug!(
                "set_layout action {:#?}",
                OnMidiAction::new(LoadLayout, midi)
            );
            layout::set_layout_by_id(db, id).await
        } else {
            Ok(id)
        }
    }

    // загрузить расклатку по индексу, сбросить редактирование
    async fn set_layout_local(&self, ctx: &Context<'_>, id: usize) -> Result<usize> {
        let db = ctx.data_unchecked::<Storage>().lock().await;
        if db.lid != id {
            layout::set_layout_by_id(db, id).await
        } else {
            Ok(id)
        }
    }

    async fn button_pressed(&self, ctx: &Context<'_>, id: usize) -> Result<Option<usize>> {
        button::action(ctx, id).await
    }

    // установаить параметр для редактирования джостиками и отправить нотайс по миди
    async fn set_edit_param(&self, ctx: &Context<'_>, id: usize) -> Result<usize> {
        param::set_edit_param_by_id(ctx, id).await
    }

    // установаить параметр для редактирования джостиками
    async fn set_edit_param_silent(&self, ctx: &Context<'_>, id: usize) -> Result<usize> {
        param::set_edit_param_by_id(ctx, id).await
    }

    // установаить скорость редактирования параметра
    async fn set_param_speed(&self, ctx: &Context<'_>, id: u8, speed: u8) -> Result<u8> {
        param::set_speed(ctx, id, speed).await
    }
}

#[Subscription]
impl SubscriptionRoot {
    // подписка на изменения кнопок раскладки
    async fn button_changed(
        &self,
        mutation: Option<button::Event>,
    ) -> impl Stream<Item = button::Mutation> {
        button::subscribe(mutation).await
    }

    // подписка на изменениe раскладки
    async fn layout_changed(
        &self,
        mutation: Option<layout::Event>,
    ) -> impl Stream<Item = layout::Mutation> {
        layout::subscribe(mutation).await
    }

    // подписка на данные для web-gui
    async fn edit_param_changed(
        &self,
        mutation: Option<param::Event>,
    ) -> impl Stream<Item = param::Mutation> {
        param::subscribe(mutation).await
    }

    // подписка на данные для оправки midi
    async fn midi_action(
        &self,
        mutation: Option<midi::MidiAction>,
    ) -> impl Stream<Item = midi::OnMidiAction> {
        midi::subscribe(mutation).await
    }

    async fn gui_action(&self, mutation: Option<gui::Event>) -> impl Stream<Item = gui::Mutation> {
        gui::subscribe(mutation).await
    }
}
