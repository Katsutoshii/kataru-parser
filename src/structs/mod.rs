mod branches;
mod comparator;
mod conditional;
mod config;
mod operator;
mod state;
mod story;
mod value;

use crate::ParseError;
pub use branches::{Branchable, Branches};
pub use comparator::Comparator;
pub use conditional::Conditional;
pub use config::{CharacterData, Config, Params};
pub use operator::Operator;
pub use state::{State, StateMod, StateUpdatable};
use std::collections::BTreeMap;
pub use story::{Choices, Cmd, Dialogue, Goto, Line, Passage, SetCmd, Story};
pub use value::Value;

pub type Map<K, V> = BTreeMap<K, V>;

pub trait Parsable<'a> {
    fn parse(text: &'a str) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized;
}
