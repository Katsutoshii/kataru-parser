#[macro_use]
mod error;

mod deserializer;
mod loader;
mod packer;
mod structs;
mod validator;

pub use deserializer::Deserializable;
pub use error::ParseError;
pub use loader::Loadable;
pub use packer::pack;
pub use structs::{
    Branchable, Branches, Choices, Cmd, Comparator, Conditional, Config, Dialogue, Goto, Line, Map,
    Operator, Params, Parsable, Passage, SetCmd, State, StateMod, StateUpdatable, Story, Value,
};
pub use validator::validate;
