use super::{Map, Parsable, ParseError, State, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterData {
    pub description: String,
}

pub type Characters = Map<String, CharacterData>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub passage: String,
    pub line: usize,
    pub state: State,
    pub cmds: Map<String, Map<String, Value>>,
    pub characters: Characters,
}

impl Parsable<'_> for Config {
    fn parse(text: &str) -> Result<Self, ParseError> {
        match serde_yaml::from_str(text) {
            Ok(config) => Ok(config),
            Err(e) => Err(perror!("{}", e)),
        }
    }
}
