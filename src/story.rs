use super::{Branches, Map, Parsable, ParseError, State, Value};
use serde::{Deserialize, Serialize};

pub type Dialogue = Map<String, String>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choices {
    pub choices: Map<String, String>,
    pub timeout: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cmd {
    pub cmd: String,
    pub params: Option<Map<String, Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Goto {
    pub goto: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCmd {
    pub set: State,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Line {
    Branches(Branches),
    Choices(Choices),
    Goto(Goto),
    Text(String),
    SetCmd(SetCmd),
    Cmd(Cmd),
    Dialogue(Dialogue),
    Continue,
    Break,
    InvalidChoice,
}

pub type Passage = Vec<Line>;

pub type Story = Map<String, Passage>;

impl Parsable<'_> for Story {
    fn parse(text: &str) -> Result<Self, ParseError> {
        match serde_yaml::from_str(text) {
            Ok(story) => Ok(story),
            Err(e) => Err(perror!("{}", e)),
        }
    }
}
