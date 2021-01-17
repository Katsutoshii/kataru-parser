use super::{
    Branches, Choices, Cmd, Comparator, Conditional, Config, Line, Map, Operator, Parsable,
    ParseError, Passage, State, StateMod, Story, Value,
};
use html_parser::Dom;

/// Validate text to guarantee valid HTML.
fn validate_text(text: &str) -> Result<(), ParseError> {
    match Dom::parse(text) {
        Err(e) => Err(perror!("Text error: {}", e)),
        Ok(_) => Ok(()),
    }
}

/// Validate that the dialogue contains valid text and configured characters only.
fn validate_dialogue(config: &Config, dialogue: &Map<String, String>) -> Result<(), ParseError> {
    for (name, text) in dialogue {
        if !config.characters.contains_key(name) {
            return Err(perror!("Undefined character name: {}", name));
        }
        validate_text(&text)?;
    }
    Ok(())
}

// Validates a conditional.
fn validate_conditional(
    config: &Config,
    story: &Story,
    branches: &Branches,
) -> Result<(), ParseError> {
    for (expression, lines) in branches {
        if expression != "else" {
            let cond = Conditional::parse(expression)?;
            if !config.state.contains_key(cond.var) {
                return Err(perror!("No such state '{}'.", cond.var));
            }
            cond.eval(&config.state)?;
            validate_cmp(&cond.val, &config.state[cond.var], cond.cmp)?;
        }
        validate_passage(config, story, lines)?;
    }
    Ok(())
}

fn validate_params(cmd: &Cmd, config_params: &Map<String, Value>) -> Result<(), ParseError> {
    match &cmd.params {
        Some(params) => {
            for (param, _val) in params {
                if !config_params.contains_key(param) {
                    return Err(perror!(
                        "No such parameter '{}' for command '{}'",
                        param,
                        cmd.cmd
                    ));
                }
            }
            Ok(())
        }
        None => Ok(()),
    }
}
fn validate_cmd(config: &Config, cmd: &Cmd) -> Result<(), ParseError> {
    if !config.cmds.contains_key(&cmd.cmd) {}
    match config.cmds.get(&cmd.cmd) {
        None => Err(perror!("No such command '{}'.", cmd.cmd)),
        Some(config_params) => validate_params(cmd, config_params),
    }
}
/// Validates a line of dialogue.
fn validate_line(config: &Config, story: &Story, line: &Line) -> Result<(), ParseError> {
    match &line {
        Line::Dialogue(dialogue) => validate_dialogue(config, dialogue),
        Line::Text(text) => validate_text(text),
        Line::Branches(cond) => validate_conditional(config, story, cond),
        Line::Choices(choices) => validate_choices(story, choices),
        Line::Goto(goto) => validate_goto(story, &goto.goto),
        Line::SetCmd(cmd) => validate_state(config, &cmd.set),
        Line::Cmd(cmd) => validate_cmd(config, &cmd),
        _ => Ok(()),
    }
}

/// Validates that a line (either text or dialogue) has valid HTML and valid speakers.
fn validate_passage(config: &Config, story: &Story, lines: &Passage) -> Result<(), ParseError> {
    for (i, line) in lines.iter().enumerate() {
        if let Err(e) = validate_line(config, story, line) {
            return Err(perror!("Line {}: {}", i + 1, e));
        }
    }
    Ok(())
}

/// Validates an operator on a given value.
/// Any value supports assignment, but only Numbers can be added or subtracted.
fn validate_op(v1: &Value, v2: &Value, op: Operator) -> Result<(), ParseError> {
    match op {
        Operator::SET => {
            if v1.same_type(v2) {
                Ok(())
            } else {
                Err(perror!(
                    "Operators require operands of the same type, not {:?} and {:?}",
                    v1,
                    v2
                ))
            }
        }
        Operator::ADD | Operator::SUB => match (v1, v2) {
            (Value::Number(_), Value::Number(_)) => Ok(()),
            _ => Err(perror!(
                "Comparators '+,-' can only be used on two numbers, not {:?} and {:?}.",
                v1,
                v2
            )),
        },
    }
}

/// Validates an comparator on a given value.
/// Any value supports assignment, but only Numbers can be added or subtracted.
fn validate_cmp(v1: &Value, v2: &Value, cmp: Comparator) -> Result<(), ParseError> {
    match cmp {
        Comparator::EQ | Comparator::NEQ => {
            if v1.same_type(v2) {
                Ok(())
            } else {
                Err(perror!(
                    "Comparisons require values of the same type, not {:?} and {:?}",
                    v1,
                    v2
                ))
            }
        }
        Comparator::LT | Comparator::LEQ | Comparator::GT | Comparator::GEQ => match (v1, v2) {
            (Value::Number(_), Value::Number(_)) => Ok(()),
            _ => Err(perror!(
                "Comparators '>,>=,<,<=' can only be used between two numbers, not {:?} and {:?}.",
                v1,
                v2
            )),
        },
    }
}

fn validate_state_var(config: &Config, var: &str) -> Result<(), ParseError> {
    if !config.state.contains_key(var) {
        Err(perror!("No state variable named '{}'", var))
    } else {
        Ok(())
    }
}
/// Validates the state only contains configured keys.
fn validate_state(config: &Config, state: &State) -> Result<(), ParseError> {
    for (key, value) in state {
        let smod = StateMod::parse(key)?;
        validate_state_var(config, smod.var)?;
        validate_op(&config.state[smod.var], value, smod.op)?;
    }
    Ok(())
}

fn validate_goto(story: &Story, passage_name: &str) -> Result<(), ParseError> {
    if !story.contains_key(passage_name) {
        Err(perror!(
            "Passage name '{}' was not defined in the story.",
            passage_name
        ))
    } else {
        Ok(())
    }
}

/// Validates that the story contains the referenced passage.
fn validate_choices(story: &Story, choices: &Choices) -> Result<(), ParseError> {
    for (_choice, passage_name) in &choices.choices {
        validate_goto(story, passage_name)?;
    }
    Ok(())
}

// Validates an entire story for valid passage references, HTML, conditionals.
pub fn validate(config: &Config, story: &Story) -> Result<(), ParseError> {
    for (passage_name, passage) in story {
        if let Err(e) = validate_passage(config, story, passage) {
            return Err(perror!("Passage '{}': {}", passage_name, e));
        }
    }
    Ok(())
}
