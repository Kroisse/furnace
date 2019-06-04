#![warn(clippy::main)]
#![deny(rust_2018_idioms)]

pub mod component;
pub mod dispatcher;
pub mod error;
pub mod model;
pub mod prelude;

pub use crate::{
    component::Component,
    dispatcher::{Dispatcher, Handle},
    error::{Error, Fallible},
    model::{Action, Update},
};
