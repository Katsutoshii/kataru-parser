#[macro_use]
mod error;

mod loader;
mod packer;
mod structs;
mod validator;

pub use error::ParseError;
pub use loader::Loadable;
pub use packer::{pack, unpack};
pub use structs::{
    Branchable, Branches, Choices, Cmd, Comparator, Conditional, Config, Dialogue, Line, Map,
    Operator, Params, Parsable, Passage, State, StateMod, StateUpdatable, Story, Value,
};
pub use validator::validate;
