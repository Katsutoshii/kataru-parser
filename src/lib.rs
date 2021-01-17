#[macro_use]
mod error;
mod branches;
mod comparator;
mod conditional;
mod config;
mod operator;
mod state;
mod story;
mod validator;
mod value;

use std::collections::BTreeMap;

pub use crate::error::ParseError;
pub use branches::{Branchable, Branches};
pub use comparator::Comparator;
pub use conditional::Conditional;
pub use config::Config;
pub use operator::Operator;
pub use state::{State, StateMod, StateUpdatable};
pub use story::{Choices, Cmd, Dialogue, Line, Passage, Story};
pub use validator::validate;
pub use value::Value;

pub type Map<K, V> = BTreeMap<K, V>;

pub trait Parsable<'a> {
    fn parse(text: &'a str) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized;
}
