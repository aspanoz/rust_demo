#[macro_use]
extern crate log;

use async_graphql::{Enum, Schema};
use futures::lock::Mutex;
use std::sync::Arc;
use strum_macros::{Display, EnumString};

pub mod schema;
mod subscribe;

#[derive(Enum, Eq, PartialEq, Copy, Clone, Debug, EnumString, Display)]
pub enum Action {
    #[strum(serialize = "LOAD_LAYOUT")]
    LoadLayout,
    #[strum(serialize = "EDIT_PARAM")]
    EditParam,
    #[strum(serialize = "SELECT_CHAN")]
    SelectChan,
    #[strum(serialize = "NONE")]
    None,
}

pub type DataSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub type Storage = Arc<Mutex<schema::Data>>;

pub struct QueryRoot;

pub struct MutationRoot;

pub struct SubscriptionRoot;
